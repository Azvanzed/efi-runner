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
2. Create a new partition on a FAT32 device like a USB, check [How to create a partition](https://github.com/Azvanzed/efi-runner/edit/main/README.md#how-to-add-device-to-vmware).
3. Now add that device to vmware, check [How to add device to VMWare](https://github.com/Azvanzed/efi-runner/edit/main/README.md#how-to-add-device-to-vmware).
4. Change your boot order so you always boot on the partition, check [How to change boot order](https://github.com/Azvanzed/efi-runner/edit/main/README.md#how-to-add-device-to-vmware).
5. Now you can run the efi-runner with the settings you want, you could configure it with your IDE so all you have to do is press a run button.

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

### How to change boot order
![Boot into the firmware](https://i.imgur.com/D2a4QPu.png)
![Enter into setup](https://i.imgur.com/Ov5VkyG.png)
![Go in boot options](https://i.imgur.com/d1ws5H5.png)
![Go in Change boot order](https://i.imgur.com/d1ws5H5.png)
![Change boot order](https://i.imgur.com/FTazE24.png)
![Commit](https://i.imgur.com/MNBe8XT.png)

## Logging
Previously, it supported serial logging but it was removed due to it being redundant. 
Now it uses VMWare backdoor logging.

## Help
Open an issue and i'll try my best to help you out.
