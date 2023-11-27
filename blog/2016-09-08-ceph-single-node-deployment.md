date = "2016-09-08T00:00:00Z"
summary = "Setting up a CEPH cluster on a single host for testing purposes"
tags = [
  "homelab",
  "ceph",
  "cobbler",
  "ansible",
  "libvirt",
]
title = "CEPH Single Node Deployment"
---

I wanted to try out Ceph at home, but hesitated, because it usually requires several physical boxes containing several disks to tap its full potential. Still, because the old virtualized ZFS setup proved quite complicated and inflexible, I decided to give Ceph a try. In the end, it still proved to be complicated, but offeres many unique benefits, such as easy storage expansion as well as efficient and fine-tuneable space usage.

For this, we will run the Ceph daemons, mainly monitors and OSDs, in separate virtual machines managed by `libvirt`. The cluster will then provide RBDs to libvirt, used to store the other virtual machines.

The whitebox is built with a Intel i3-4160 processor and 32GB of ECC RAM. The Supermicro motherboard has a dedicated IPMI port. Concerning storage, we got 2 SSDs with 128GB each, and two HDDs with 3TB capacity, respectively. The box also got 2 Intel Gigabit NICs, which will be set up in a bond configuration for high availability. The two SSDs will be set up in a RAID 1 and contain the hypervisor OS, the backing volumes for the virtual machines comprising the Ceph cluster, and the Ceph journals.

`CentOS 7` will be used both for the hypervisor and for all virtual machines.

At the end of this post, we will have a Ceph cluster running, consisting of 3 monitors and 2 OSDs, each OSD handling one of the HDDs.

# Ceph

I will not describe Ceph here, because the official website provides everything you have to know specifc to Ceph: http://ceph.com/

# Preparations

## Network overview

We will be using the followings networks for our cluster:

| Network | Name | VLAN | Domain |
| --- | --- | ---:| ---:|
| `10.1.2.0/24` | `mgmt` | `20` | `mgmt.home.haktec.de` |
| `10.1.3.0/24` | `storage` | `30` | `storage.home.haktec.de` |
| `10.1.4.0/24` | `cluster` | isolated on the hypervisor ||

| IP | Host |
| --- | --- |
| `10.1.{3,4}.11-` | Ceph monitors, the first one being `.11`, second one `.12`, and so on |
| `10.1.{3,4}.21-` | Ceph storage servers, same scheme as above, but from `.21` |

| IP | Host |
| --- | --- |
| `10.1.{2,3}.100` | Admin node |
| `10.1.2.10` | hypervisor |

| Hostname | Host |
| --- | --- |
| `hyper01` | hypervisor |
| `ceph-monXX` | Ceph monitors |
| `ceph-stoXX` | Ceph storage servers |

## Storage configuration

The two SSDs will be RAIDed together, which gives 128GB of available space. The hypervisor will need a bit more than 20GiB:

```
512 MiB /boot
 10 GiB /
 10 GiB /var
  1 GiB swap
```

Each Ceph VM gets 10GiB of storage, partitioned like this:

```
512 MiB /boot
  4 GiB /
  5 GiB /var
512 MiB swap
```

Also, both OSD journal have a size of 1GiB each. All in all, this amounts to the following:

```
hypervisor       | 1 x 21GiB | 21GiB
virtual machines | 5 x 10GiB | 50GiB
journals         | 2 x  1GiB |  2GiB
------------------------------------
                               73GiB
```

This gives around 40GiB of remaining space, which would allow for 2 more OSDs in the future. By chance, there are exactly 2 SATA ports unused ... we will see ;)

# Software and Deployment

We will use Cobbler as an install server to provision both the hypervisor and the virtual machines, and then use Ansible to configure them.

The admin machine that runs Cobbler, Ansible and the Ceph install scripts is a simply my notebook running Arch Linux. I will not describe setup and administration of that machine, because it should be kept as distro-agnostic as possible. Just make sure Cobbler, Ansible and Ceph are installed on the admin machine, whatever you use. You are expected to know what you are doing ;)

# Installing the hypervisor

Ok, enough theory. The first thing we have to do is set up the hypervisor, which is a standard CentOS 7 setup procedure.

## Setting up Cobbler

First, cobbler needs to be set up for the `mgmt` network. In `/etc/cobbler/settings`, the following options need to be set:

```
admin : /etc/cobbler/settings
```
```yaml
manage_dhcp: 1
manage_dns: 1
manage_tftpd: 1
manage_rsync: 0

next_server: 10.1.2.100
server: 10.1.2.100
```

```
admin : /etc/cobbler/dhcp.template
```
```
[...]
subnet 10.1.2.0 netmask 255.255.255.0 {
     option routers             10.1.2.1;
     option domain-name-servers 10.1.2.1;
[...]
```

* `manage_dhcp`:
Tells Cobbler to use `dhcpd` and hand out addresses
* `manage_dns`:
Makes Cobbler use the BIND DNS server
* `manage_tftpd`:
`tftpd` is needed to get the necessary boot files to the booting clients
* `next_server`:
Needs to be set to the IP of the host running Cobbler, pushed out to clients via DHCP, necessary for clients to reach out to cobbler when PXE booting
* `server`:
This is simply the IP address of the Cobbler host, used by booting clients, for example to retrieve kickstart files

The password hash can be produced like this:

```shell
[~]$ python3 -c 'import crypt; print(crypt.crypt("cleartext", crypt.mksalt(crypt.METHOD_SHA512)))'
```

Start cobbler and do a first `sync`:

```bash
[~]$ sudo systemctl start cobblerd
[~]$ sudo systemctl enable cobblerd
[~]$ sudo cobbler sync
```

Now we need to import a distribution from the CentOS 7 iso:

```bash
[~]$ sudo mount -t iso9660 -o loop,ro /path/to/iso/CentOS7.iso /mnt
[~]$ sudo cobbler import --name centos7 --arch=x86_64 --path=/mnt
```

## Kickstarting the hypervisor

The following kickstart will be used for the hypervisor:

```
/var/lib/cobbler/kickstarts/hyper01.ks
```
```shell
# AUTHENTICATION AND USERS
auth --enableshadow --passalgo=sha512
rootpw --iscrypted $rootpw

# FIREWALL
firewall --disabled

# LOCALIZATION AND TIME
keyboard --vckeymap=de-nodeadkeys
lang en_US.UTF-8
timezone --nontp --utc Europe/Berlin --nontp

# MISC
selinux --enforcing

# SERVICES
services --disabled=firewalld,NetworkManager

# INSTALLATION MEDIUM
cdrom

# INSTALLATION MODE
install
skipx
text
poweroff

# NETWORK
network --hostname=hyper01.mgmt.haktec.de
network --activate --onboot=yes --device=00:25:90:47:6e:14 --noipv4 --noipv6
network --activate --onboot=yes --device=00:25:90:47:6e:15 --noipv4 --activate
network --activate --onboot=yes --device=bond0 --noipv4 --noipv6 --bondslaves=eno1,eno2 --bondopts=mode=active-backup,miimon=100
network --activate --onboot=yes --device=bond0.30 --noipv4 --noipv6 --vlanid=30
network --activate --onboot=yes --device=br-home --bridgeslaves=bond0.30 --bootproto=dhcp

# PARTITIONS
ignoredisk --only-use=/dev/disk/by-id/ata-SanDisk_SDSSDP128G_150610400358,/dev/disk/by-id/ata-SanDisk_SDSSDP128G_152964400540

zerombr

bootloader --location=mbr --boot-drive=/dev/disk/by-id/ata-SanDisk_SDSSDP128G_150610400358
clearpart --all --initlabel

part raid.01 --fstype=mdmember --ondisk=/dev/disk/by-id/ata-SanDisk_SDSSDP128G_150610400358 --size=512
part raid.02 --fstype=mdmember --ondisk=/dev/disk/by-id/ata-SanDisk_SDSSDP128G_152964400540 --size=512

part raid.11 --fstype=mdmember --ondisk=/dev/disk/by-id/ata-SanDisk_SDSSDP128G_150610400358 --grow
part raid.12 --fstype=mdmember --ondisk=/dev/disk/by-id/ata-SanDisk_SDSSDP128G_152964400540 --grow

raid /boot --level=1 --device=boot --fstype=xfs   raid.01 raid.02
raid pv.01 --level=1 --device=pv01 --fstype=lvmpv raid.11 raid.12

volgroup vg.hyper01 pv.01

logvol /    --vgname=vg.hyper01 --name=root --size=5120 --fstype=xfs
logvol /var --vgname=vg.hyper01 --name=var  --size=4096 --fstype=xfs
logvol swap --vgname=vg.hyper01 --name=swap --size=1024 --fstype=xfs

%packages
@Core
bridge-utils
tmux
-firewalld
-NetworkManager
%end

%post --erroronfail

# create sudo group
groupadd --system sudo
echo '%sudo ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers

# create initial user
useradd --create-home --home-dir /home/$me --groups sudo --user-group $me

# Disable password and root login for SSH
sed -i -e 's/^PasswordAuthentication yes/PasswordAuthentication no/g' /etc/ssh/sshd_config
echo 'PermitRootLogin no' >> /etc/ssh/sshd_config

mkdir --parents --mode 700 /home/$me/.ssh

cat > /home/$me/.ssh/authorized_keys << EOF
$sshkey
EOF

chmod 600 /home/$me/.ssh/authorized_keys
chown -R $me:$me /home/$me
%end
```

Some global variables, e.g. SSH keys, need to be set. This is done in the profile, so it will be inherited by all other systems we define:

```shell
[~]$ sudo cobbler profile edit \
    --name=centos7-x86_64 \
    --ksmeta='rootpw={hashed_password} sshkey={ssh_pubkey} me=hannes'
```

The following will create a new system entry for the hypervisor:

```shell
[~]$ sudo cobbler system add \
    --name=hyper01.mgmt.haktec.de \
    --profile=centos7-x86_64 \
    --kickstart=/var/lib/cobbler/kickstarts/hyper01.ks \
    --interface=eno1 \
    --mac=00:25:90:47:6e:14 \
    --ip-address=10.1.2.10
```

Now, the physical machine is started, PXE boots and the installation finishes on its own.

# Setting up the hypervisor

Using Ansible, we configure the network of the hypervisor. We bond `eno1` and `eno2` together,
and enable access to the `mgmt` network (`VLAN 20`):

```
hyper01 : /etc/sysconfig/networks-scripts/ifcfg-bond0
```
```shell
DEVICE=bond0
TYPE=Bond
ONBOOT=yes
BOOTPROTO=none
USERCTL=no
BONDING_OPTS="mode=1 miimon=100"
```

```
hyper01 : /etc/sysconfig/networks-scripts/ifcfg-eno1
```
```shell
DEVICE=eno1
ONBOOT=yes
BOOTPROTO=none
USERCTL=no
MASTER=bond0
SLAVE=yes
```

```
hyper01 : /etc/sysconfig/networks-scripts/ifcfg-eno2
```
```shell
DEVICE=eno2
ONBOOT=yes
BOOTPROTO=none
USERCTL=no
MASTER=bond0
SLAVE=yes
```

```
hyper01 : /etc/sysconfig/networks-scripts/ifcfg-bond0.20
```
```shell
DEVICE=bond0.20
ONBOOT=yes
BOOTPROTO=none
TYPE=None
VLAN=yes
USERCTL=no
BRIDGE=br-mgmt
```

```
hyper01 : /etc/sysconfig/networks-scripts/ifcfg-br-mgmt
```
```shell
DEVICE=br-mgmt
ONBOOT=yes
TYPE=Bridge
BOOTPROTO=dhcp
```

After this, the `network` service has to be restarted:

```shell
[~]$ sudo systemctl restart network
```

Check the RAID Setup:

```shell
[~]$ sudo cat /proc/mdstat
Personalities : [raid1]
md126 : active raid1 sdb2[0] sda2[1]
      122489856 blocks super 1.2 [2/2] [UU]
      bitmap: 0/1 pages [0KB], 65536KB chunk

md127 : active raid1 sdb1[0] sda1[1]
      524224 blocks super 1.0 [2/2] [UU]
      bitmap: 0/1 pages [0KB], 65536KB chunk

unused devices: <none>
```

Update the system:

```shell
[~]$ sudo yum update
```

Enable passwordless sudo:

```shell
[~]$ sudo groupadd --system sudo
[~]$ sudo echo '%sudo ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers
[~]$ sudo gpasswd -a hannes sudo
```

Disable SSH Password Authentication:

```shell
[~]$ sudo sed -i -e 's/^PasswordAuthentication yes/PasswordAuthentication no/g' /etc/ssh/sshd_config
[~]$ sudo systemctl restart sshd
```

# Setting up libvirt

```shell
[~]$ sudo yum install libvirt qemu-kvm
```

Everyone in group `libvirt` is allowed to access libvirt:

```shell
[~]$ sudo gpasswd -a hannes libvirt
```

Start libvirt:

```shell
[~]$ sudo systemctl start libvirtd
[~]$ sudo systemctl enable libvirtd
```

Connect to the hypervisor:

```shell
[~]$ virsh --connect=qemu:///system
```

## libvirt storage setup

```
~/pool-lvm-hyper01.xml
```

```xml
<pool type='logical'>
  <name>lvm-hyper01</name>
  <source>
    <name>vg.hyper01</name>
  </source>
  <target>
    <path>/dev/vg.hyper01</path>
  </target>
</pool>
```

Define and start the pool:

```shell
[~]$ sudo virsh pool-define pool-lvm-hyper01.xml
[~]$ sudo virsh pool-start lvm-hyper01
[~]$ sudo virsh pool-autostart lvm-hyper01
[~]$ sudo virsh pool-list --all --details
 Name         State    Autostart  Persistent    Capacity  Allocation   Available
---------------------------------------------------------------------------------
 lvm-hyper01  running  yes        yes         116.81 GiB   10.00 GiB  106.81 GiB
```

## libvirt network setup

2 networks: storage and cluster

Remove the `default` network:

```
[~]$ sudo virsh net-destroy default
[~]$ sudo virsh net-undefine default
```

Define the relevant bridges:

```
hyper01 : /etc/sysconfig/networks-scripts/ifcfg-br-cluster
```
```shell
DEVICE=br-cluster
ONBOOT=yes
TYPE=Bridge
BOOTPROTO=none
```

```
hyper01 : /etc/sysconfig/networks-scripts/ifcfg-br-storage
```
```shell
DEVICE=br-storage
ONBOOT=yes
TYPE=Bridge
BOOTPROTO=static
IPADDR=10.3.1.1
NETMASK=255.255.255.0
```

```shell
hyper01[~]$ sudo systemctl restart network
```

Network definitions:
```
~/network-cluster.xml
```
```xml
<network ipv6='yes'>
  <name>cluster</name>
  <forward mode='bridge'/>
  <bridge name='br-cluster'/>
</network>
```

```
~/network-storage.xml
```
```xml
<network ipv6='yes'>
  <name>storage</name>
  <forward mode='bridge'/>
  <bridge name='br-storage'/>
</network>
```

Start the networks:

```shell
[~]$ sudo virsh net-define network-cluster.xml
[~]$ sudo virsh net-define network-storage.xml

[~]$ sudo virsh net-start cluster
[~]$ sudo virsh net-start storage

[~]$ sudo virsh net-autostart cluster
[~]$ sudo virsh net-autostart storage

[~]$ sudo virsh net-list --all
 Name                 State      Autostart     Persistent
----------------------------------------------------------
 cluster              active     yes           yes
 storage              active     yes           yes
```

# The VMS

## Creating the Ceph VMs

## Create the first monitor VM

Create the storage volume:

```shell
[~]$ sudo virsh vol-create-as --pool lvm-hyper01 --name virt-mon01 --capacity 10GiB --format raw
```

```shell
[~]$ sudo lvdisplay vg.hyper01/virt-mon01
  --- Logical volume ---
  LV Path                /dev/vg.hyper01/virt-mon01
  LV Name                virt-mon01
  VG Name                vg.hyper01
  LV UUID                6LLfnI-Jrve-q5IV-dBER-w0jv-wt8Y-TBAKf5
  LV Write Access        read/write
  LV Creation host, time hyper01.mgmt.haktec.de, 2016-09-09 23:52:16 +0200
  LV Status              available
  # open                 0
  LV Size                10.00 GiB
  Current LE             2560
  Segments               1
  Allocation             inherit
  Read ahead sectors     auto
  - currently set to     256
  Block device           253:3
```

Domain definition:

```
~/domain-ceph-mon01.xml
```
```xml
<domain type='kvm'>
  <name>ceph-mon01</name>
  <memory unit='KiB'>1048576</memory>
  <currentMemory unit='KiB'>1048576</currentMemory>
  <vcpu placement='static'>1</vcpu>
  <os>
    <type arch='x86_64'>hvm</type>
    <boot dev='cdrom'/>
    <boot dev='hd'/>
  </os>
  <features>
    <acpi/>
    <apic/>
  </features>
  <clock offset='utc'>
    <timer name='rtc' tickpolicy='catchup'/>
  </clock>
  <on_poweroff>destroy</on_poweroff>
  <on_reboot>restart</on_reboot>
  <on_crash>destroy</on_crash>
  <devices>
    <disk type='volume' device='disk'>
      <driver name='qemu' type='raw' cache='none' io='native' discard='unmap'/>
      <source pool='lvm-hyper01' volume='virt-mon01'/>
      <target dev='vda' bus='virtio'/>
    </disk>
    <interface type='network'>
      <source network='cluster'/>
      <model type='virtio'/>
      <driver name='vhost'/>
    </interface>
    <interface type='network'>
      <source network='storage'/>
      <model type='virtio'/>
      <driver name='vhost'/>
    </interface>
    <input type='tablet' bus='usb'/>
    <input type='keyboard' bus='usb'/>
    <channel type='spicevmc'>
      <target type='virtio'/>
    </channel>
    <graphics type='spice' autoport='yes' listen='127.0.0.1'>
      <listen type='address' address='127.0.0.1'/>
    </graphics>
    <video>
      <model type='qxl'/>
    </video>
  </devices>
</domain>
```

Define the VM:

```shell
[~]$ sudo virsh define domain-ceph-mon01.xml
```

Attach the ISO for installation:

```shell
[~]$ sudo virsh attach-disk \
    --domain ceph-mon01 \
    --source /var/lib/libvirt/iso/CentOS-7-x86_64-Minimal-1511.iso \
    --target vdz \
    --targetbus virtio \
    --config
```

Start the VM:

```shell
[~]$ sudo virsh start ceph-mon01
```

Install CentOS, kickstart:

```shell
# AUTHENTICATION AND USERS
auth --enableshadow --passalgo=sha512
rootpw --iscrypted {{password}}

# FIREWALL
firewall --disabled

# LOCALIZATION AND TIME
keyboard --vckeymap=de-nodeadkeys
lang en_US.UTF-8
timezone --nontp --utc Europe/Berlin --nontp

# MISC
selinux --enforcing

# WHERE TO INSTALL FROM
install
cdrom

skipx
text
poweroff

services --disabled=firewalld,NetworkManager

# NETWORK
network --hostname=ceph-mon01.storage.haktec.de
network --activate --onboot=yes --device=52:54:00:ba:5a:db --bootproto=dhcp --noipv6

# PARTITIONS
ignoredisk --only-use=vda

zerombr

bootloader --location=mbr --boot-drive=vda
clearpart --drives=vda --initlabel

part /boot --ondrive=vda --size 512 --fstype=xfs --label=boot
part pv.01 --ondrive=vda --grow

volgroup ceph-mon01 pv.01

logvol /    --vgname=ceph-mon01 --name=root --size=4096 --fstype=xfs
logvol /var --vgname=ceph-mon01 --name=var  --size=1024 --fstype=xfs --grow
logvol swap --vgname=ceph-mon01 --name=swap --size=512

user --name hannes

%packages
@Core
-firewalld
-NetworkManager
%end

%post --erroronfail
mkdir --parents --mode 700 /home/hannes/.ssh

cat > /home/hannes/.ssh/authorized_keys << EOF
ssh-rsa {{pubkey}} hannes
EOF

chmod 600 /home/hannes/.ssh/authorized_keys
chown -R hannes:hannes /home/hannes
%end
```

How to make the kickstart file available from the hypervisor:

```shell
[~]$ while :; do nc -l 8080 < ks.cfg ; done
```

```shell
[~]$ sudo dnsmasq -d -i br-storage -I lo --bind-interfaces --dhcp-range=10.3.1.100,10.3.1.199,255.255.255.0 -C /dev/null
```

Install the guest and wait for it to shut down.

Detach the ISO:

```shell
[~]$ sudo virsh detach-disk --domain ceph-mon01 --target vdz --config
```

Start it again in order to set it up:

```shell
[~]$ sudo virsh start ceph-mon01
```

Log in, and do the usual deployment stuff.

Network setup:


```
ceph-mon01 : /etc/sysconfig/networks-scripts/ifcfg-eth0
```
```shell
DEVICE=eth0
ONBOOT=yes
USERCTL=no
BOOTPROTO=static
IPADDR=10.4.1.10
NETMASK=255.255.255.0
HWADDR=52:54:00:ba:5a:db
```

```
ceph-mon01 : /etc/sysconfig/networks-scripts/ifcfg-eth1
```
```shell
DEVICE=eth1
ONBOOT=yes
USERCTL=no
BOOTPROTO=static
IPADDR=10.3.1.10
NETMASK=255.255.255.0
GATEWAY=10.3.1.1
HWADDR=52:54:00:3b:2c:3a
```

```shell
ceph-mon01[~]$ sudo systemctl restart network
```

From above:

```shell
ceph-mon01[~]$ sudo yum update

ceph-mon01[~]$ sudo groupadd --system sudo
ceph-mon01[~]$ sudo echo '%sudo ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers
ceph-mon01[~]$ sudo gpasswd -a hannes sudo

ceph-mon01[~]$ sudo sed -i -e 's/^PasswordAuthentication yes/PasswordAuthentication no/g' /etc/ssh/sshd_config
ceph-mon01[~]$ sudo systemctl restart sshd
```

Let's create a user called `cephdeploy` that has passwordless sudo rights and SSH Pubkey authentication:

```shell
ceph-mon01[~]$ sudo useradd --create-home --groups sudo cephdeploy
ceph-mon01[~]$ sudo echo "ssh-rsa {{pubkey}} hannes" > /home/cephdeploy/.ssh/authorized_keys
ceph-mon01[~]$ sudo chmod -R go= /home/cephdeploy/.ssh/
ceph-mon01[~]$ sudo chown -R cephdeploy:cephdeploy /home/cephdeploy/
```

The following is necessary for `ceph-deploy` to work:

```shell
ceph-mon01[~]$ sudo echo 'Defaults:cephdeploy !requiretty' >> /etc/sudoers
```

Install Ceph:

```shell
ceph-mon01[~]$ sudo yum install epel-release
ceph-mon01[~]$ sudo yum install ceph
```

Because we will use this VM as the base for the two OSD VMs, some cleanup is helpful:


### Cloning the Machines

Define two more machines in libvirt:

```shell
hyper01[~]$ sudo virsh vol-create-as --pool lvm-hyper01 --name virt-stg01 --capacity 10GiB --format raw
hyper01[~]$ sudo virsh vol-create-as --pool lvm-hyper01 --name virt-stg02 --capacity 10GiB --format raw

hyper01[~]$ sudo virsh vol-create-as --pool lvm-hyper01 --name virt-stg01-journal --capacity 5GiB --format raw
hyper01[~]$ sudo virsh vol-create-as --pool lvm-hyper01 --name virt-stg02-journal --capacity 5GiB --format raw

hyper01[~]$ sudo dd bs=4M status=progress if=/dev/vg.hyper01/virt-mon01 of=/dev/vg.hyper01/virt-stg01
hyper01[~]$ sudo dd bs=4M status=progress if=/dev/vg.hyper01/virt-mon01 of=/dev/vg.hyper01/virt-stg02
```

Domain definitions:

```
~/domain-ceph-stg01.xml
```
```xml
<domain type='kvm'>
  <name>ceph-stg01</name>
  <memory unit='KiB'>1048576</memory>
  <currentMemory unit='KiB'>1048576</currentMemory>
  <vcpu placement='static'>1</vcpu>
  <os>
    <type arch='x86_64'>hvm</type>
    <boot dev='cdrom'/>
    <boot dev='hd'/>
  </os>
  <features>
    <acpi/>
    <apic/>
  </features>
  <clock offset='utc'>
    <timer name='rtc' tickpolicy='catchup'/>
  </clock>
  <on_poweroff>destroy</on_poweroff>
  <on_reboot>restart</on_reboot>
  <on_crash>destroy</on_crash>
  <devices>
    <disk type='volume' device='disk'>
      <driver name='qemu' type='raw' cache='none' io='native' discard='unmap'/>
      <source pool='lvm-hyper01' volume='virt-stg01'/>
      <target dev='vda' bus='virtio'/>
    </disk>
    <disk type='block' device='disk'>
      <driver name='qemu' type='raw' cache='none' io='native'/>
      <source dev='/dev/disk/by-id/ata-WDC_WD30EFRX-68EUZN0_WD-WMC4N0H6C3U2'/>
      <target dev='vdb' bus='virtio'/>
    </disk>
    <disk type='volume' device='disk'>
      <driver name='qemu' type='raw' cache='none' io='native' discard='unmap'/>
      <source pool='lvm-hyper01' volume='virt-stg01-journal'/>
      <target dev='vdc' bus='virtio'/>
    </disk>
    <interface type='network'>
      <source network='cluster'/>
      <model type='virtio'/>
      <driver name='vhost'/>
    </interface>
    <interface type='network'>
      <source network='storage'/>
      <model type='virtio'/>
      <driver name='vhost'/>
    </interface>
    <input type='tablet' bus='usb'/>
    <input type='keyboard' bus='usb'/>
    <channel type='spicevmc'>
      <target type='virtio'/>
    </channel>
    <graphics type='spice' autoport='yes' listen='127.0.0.1'>
      <listen type='address' address='127.0.0.1'/>
    </graphics>
    <video>
      <model type='qxl'/>
    </video>
  </devices>
</domain>
```

Analogous for `ceph-stg02`.

```shell
hyper01[~]$ sudo virsh define domain-ceph-stg01.xml
hyper01[~]$ sudo virsh define domain-ceph-stg02.xml

hyper01[~]$ sudo virsh start ceph-stg01
hyper01[~]$ sudo virsh start ceph-stg02
```

For each server:

Update the hostname:

```shell
ceph-stg01[~]$ sudo hostnamectl set-hostname ceph-stg01.storage.haktec.de
ceph-stg02[~]$ sudo hostnamectl set-hostname ceph-stg02.storage.haktec.de
```

Clean the SSH Host Keys:

```shell
ceph-stg01[~]$ sudo rm -f /etc/ssh/ssh_host*
ceph-stg02[~]$ sudo rm -f /etc/ssh/ssh_host*
```

Adapt `/etc/sysconfig/network-scripts/ifcfg-eth0` and `/etc/sysconfig/network-scripts/ifcfg-eth1`.


## Connectivity

From the admin machine:

```shell
[~]$ ping 10.3.1.10
[~]$ ping 10.3.1.20
[~]$ ping 10.3.1.21
```

```
/etc/hosts
```
```
10.3.1.10 mon01
10.3.1.20 stg01
10.3.1.21 stg02
```

```
~/.ssh/config
```
```
Host mon01
  Hostname mon01
  User cephdeploy
  IdentityFile ~/.ssh/id_rsa_cephdeploy
Host stg01
  Hostname stg01
  User cephdeploy
  IdentityFile ~/.ssh/id_rsa_cephdeploy
Host stg02
  Hostname stg02
  User cephdeploy
  IdentityFile ~/.ssh/id_rsa_cephdeploy
```

```shell
[~]$ ssh mon01 'echo "10.3.1.20 ceph-stg01" | sudo tee -a /etc/hosts'
[~]$ ssh mon01 'echo "10.3.1.21 ceph-stg02" | sudo tee -a /etc/hosts'
[~]$ ssh stg01 'echo "10.3.1.10 ceph-mon01" | sudo tee -a /etc/hosts'
[~]$ ssh stg01 'echo "10.3.1.21 ceph-stg02" | sudo tee -a /etc/hosts'
[~]$ ssh stg02 'echo "10.3.1.10 ceph-mon01" | sudo tee -a /etc/hosts'
[~]$ ssh stg02 'echo "10.3.1.20 ceph-stg01" | sudo tee -a /etc/hosts'
```

# Bootstrapping the Cluster

Setup the cluster-specific files:

```shell
[~]$ mkdir cluster
[~]$ cd cluster
[~]$ ceph-deploy new ceph-mon01
```

There are some options that must be set in `ceph.conf` before creating the cluster:

Set the default number of replicas to `2`:

```
osd pool default size = 2
```

Allow the cluster to operate when only one replica is available:

```
osd pool default min size = 1
```

Set the default number of placement groups to `128`. This is the recommended
value for a small amount of OSDs

```
osd pool default pg num = 128
osd pool default pgp num = 128
```

Set the storage network:

```
public network = 10.3.1.0/24
```

Set the journal size to `5GiB - 2MiB`. The `2MiB` are needed for alignment purposes.

```
osd journal size = 5118
```

Install `ceph` on all participating nodes:

```shell
[~]$ ceph-deploy install ceph-mon01 ceph-stg01 ceph-stg02
```

Bootstrap the initial monitors (only one in our case):

```shell
[~]$ ceph-deploy mon create-initial
```

Bootstrap the OSDs. Here, `vdb` is the data disk and `vdc` is the journal. The
filesystem is `xfs`, because both `ext4` and `btrfs` have issues.

```shell
[~]$ ceph-deploy osd prepare --fs-type xfs --zap-disk ceph-stg01:/dev/vdb:/dev/vdc
[~]$ ceph-deploy osd prepare --fs-type xfs --zap-disk ceph-stg02:/dev/vdb:/dev/vdc

[~]$ ceph-deploy osd activate ceph-stg01:/dev/vdb1:/dev/vdc1
[~]$ ceph-deploy osd activate ceph-stg02:/dev/vdb1:/dev/vdc1
```

Note: If `osd prepare` fails due to `sgdisk`, the following commands will completely
reset the partition tables for data and journal disks:

```shell
[~]$ ssh ceph-stg01 'sudo sgdisk --zap-all /dev/vdb'
[~]$ ssh ceph-stg01 'sudo sgdisk --zap-all /dev/vdc'
[~]$ ssh ceph-stg02 'sudo sgdisk --zap-all /dev/vdb'
[~]$ ssh ceph-stg02 'sudo sgdisk --zap-all /dev/vdc'
```

Now, push the configuration to all cluster members:

```shell
[~]$ ceph-deploy admin ceph-mon01 ceph-stg01 ceph-stg02
```

There we are! Check the cluster status:

```shell
[~]$ ceph status
    cluster 7070adb1-b567-4ea3-bebd-01bd67c6c2bf
     health HEALTH_OK
     monmap e1: 1 mons at {ceph-mon01=10.3.1.10:6789/0}
            election epoch 4, quorum 0 ceph-mon01
     osdmap e29: 2 osds: 2 up, 2 in
            flags sortbitwise
      pgmap v117: 128 pgs, 1 pools, 0 bytes data, 0 objects
            76756 kB used, 5586 GB / 5586 GB avail
                 128 active+clean

```
