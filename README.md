# EFI-Runner
A simple efi runner and message logger for VMWare to make efi development easier.

## How does it work
It uses vmrun from vmware to interact with the virtualization interface.

## Requirements
VMWare
A FAT32 Storage device

## Warning
This has been tested only on windows!

## How to install
0. Get VMWare and make sure you have vmrun (its located in the VMWare folder) in your PATH.
1. Create a new Virtual Machine if you didn't have one already.
2. Create a new partition on a FAT32 device like a USB, check "How to create a partition" section.
3. Now add that device to vmware, check "How to add device to VMWare" section.
4. Change your boot order so you always boot on the partition.
5. 

### How to create a bootable partition
Create a partition with 512MB, follow: https://www.wikihow.com/Create-a-Partition

### How to add device to VMWare
After setting up your Virtual Machine with and created a new partition, do the following to map the partition as a device.
![Add hard-disk](https://i.imgur.com/ZZgG5Cc.png)

![Choose NVME](https://i.imgur.com/XwXgBtp.png)

![Choose physical disk](https://i.imgur.com/lBExls6.png)

![Set the correct device and choose use individual partition](https://i.imgur.com/fMPy8hv.png)

![Select the partition you want to use](https://i.imgur.com/EGWD86K.png)

![Press finish](https://i.imgur.com/JrHX3k8.png)

