# EFI-Runner
A simple efi runner and message logger for VMWare to make efi development easier.

# How does it work
It uses vmrun from vmware to interact with the virtualization interface.

# How to install
0. Get VMWare and make sure you have vmrun (its located in the VMWare folder) in your PATH.
1. Create a new Virtual Machine if you didn't have one already.
2. Create a new partition on a FAT32 device like a USB, check "How to create a partition" section.
3. Now add that device to vmware, check 

## How to create a bootable partition
Create a partition with 512MB, follow: https://www.wikihow.com/Create-a-Partition

## How to add device to VMWare
After setting up your Virtual Machine with and created a new partition, do the following to map the partition as a device.
https://i.imgur.com/ZZgG5Cc.png
https://i.imgur.com/XwXgBtp.png
https://i.imgur.com/lBExls6.png
https://i.imgur.com/fMPy8hv.png
https://i.imgur.com/EGWD86K.png
https://i.imgur.com/JrHX3k8.png

