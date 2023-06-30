+++
title = "Teardown"
template = "page.html"
+++

This guide shows how you would tear down a PC-9821Cx.

The only tool needed is a phillips head screw driver.

## 0. Setup

The first step when taking apart anything with electronics is to unplug the machine from the wall and place the machine on a surface that will not accumulate static. I do not recommend taking apart one of these machines on a bed or carpet.

## 1. Taking off the front cover

The front cover is the first thing you should take off. Behind the front cover lives the hard drive and if present ram expansion board.

The front cover is held in place with two pieces of plastic. There is one piece on each side. Press the plastic pieces on both sides and lightly pull the front cover forward until it comes free.

<img src="/pc98/cx/front_cover_side.jpg" style="max-width:80%; padding-left: 10%" alt="Side of PC-9821Cx front cover" title="Side of PC-9821Cx front cover"/>

## 2. Remove the hard drive and ram expansion board

After the front cover is removed, the hard drive and ram expansion are accessible.

<img src="/pc98/cx/cx_front_no_cover.jpg" style="max-width:80%; padding-left: 10%" alt="PC-9821Cx with no front cover" title="PC-9821Cx with no front cover"/>

At the bottom-left the hard drive cage is accessible. Above the hard drive is a CD-ROM drive. On the bottom right is either a ram expansion board or a metal cover. Above the ram expansion area are the floppy drive(s).

The ram expansion board can be removed by unscrewing the two screws and then pulling the ram board forward.

<img src="/pc98/cx/cx_front_ram_expansion.jpg" style="max-width:80%; padding-left: 10%" alt="PC-9821Cx with installed ram expansion board" title="PC-9821Cx with installed ram expansion board"/>

<img src="/pc98/cx/cx_ram_expansion_front.jpg" style="max-width:80%; padding-left: 10%" alt="PC-9821Cx ram expansion board" title="PC-9821Cx ram expansion board"/>

To remove the hard drive, unplug the IDE and power cables from the board at the bottom of the case. Then unscrew the bracket and slide the bracket forward out of the case.

<img src="/pc98/cx/cx_hdd_front.jpg" style="max-width:80%; padding-left: 10%" alt="PC-9821Cx hard drive bracket" title="PC-9821Cx hard drive bracket"/>

<img src="/pc98/cx/cx_hdd.jpg" style="max-width:80%; padding-left: 10%" alt="PC-9821Cx hard drive" title="PC-9821Cx hard drive"/>

## 3. Remove C-Bus cards

Before the top cover is removed, take out any installed C-Bus cards. In most PC-9821Cx there wil be a video board that will need to be removed. I have an LGY-98 installed and that also needs to be removed. Unscrew the thumb screws and then pull the C-Bus cards out. If the C-Bus has a plastic clip (TODO: Clip is wrong name) to help leverage out the card, make sure you use that.

<img src="/pc98/cx/cx_back.jpg" style="max-width:80%; padding-left: 10%" alt="Back of PC-9821Cx" title="Back of PC-9821Cx"/>

## 4. Remove top cover

The top cover is held by 5 screws: 2 on each side and then 1 on the top back.

<img src="/pc98/cx/cx_side.jpg" style="max-width:80%; padding-left: 10%" alt="Side of PC-9821Cx" title="Side of PC-9821Cx"/>

Once all the screws are removed, it should be possible to slide the top cover back a small distance. Then pull the cover up from the back and rotate forward until detached.

<img src="/pc98/cx/cx_top_cover.jpg" style="max-width:80%; padding-left: 10%" alt="Top cover of PC-9821Cx" title="Top cover of PC-9821Cx"/>

<img src="/pc98/cx/cx_removed_top_cover.jpg" style="max-width:80%; padding-left: 10%" alt="PC-9821Cx with removed top cover" title="PC-9821Cx with removed top cover"/>

## 5. Remove CPU accelerator card

This step can be skipped if your PC-9821Cx does not have a CPU accelerator.

Disconnect the cable going from the C-Bus backplane to the floppy drives. It is best to disconnect the cable from the side of the C-Bus backplane.

Remove the two screws that hold the CPU accelerator in place. Then pull the CPU accelerator card up and out of the case.

<img src="/pc98/cx/cx_cpu_top.jpg" style="max-width:80%; padding-left: 10%" alt="PC-9821Cx CPU accelerator" title="PC-9821Cx CPU accelerator"/>

## 6. Remove the back cover

The back cover needs to be removed next. There are four screws that hold on the back cover. Three on the bottom of the back cover and one next to the power connector.

<img src="/pc98/cx/cx_back_cover.jpg" style="max-width:80%; padding-left: 10%" alt="PC-9821Cx back cover" title="PC-9821Cx back cover"/>

## 7. Remove the power supply

The power supply is held in with two screws: one attached to the C-Bus case and the other on the side of the case.

<img src="/pc98/cx/cx_psu_top.jpg" style="max-width:80%; padding-left: 10%" alt="PC-9821Cx PSU top screw" title="PC-9821Cx PSU top screw"/>

<img src="/pc98/cx/cx_psu_side.jpg" style="max-width:80%; padding-left: 10%" alt="PC-9821Cx PSU side screw" title="PC-9821Cx PSU side screw"/>

One the screws are removed the power supply needs to be lifted up and rotated towards the CD drive so that the power cables can be detached. There is power cable that goes to the motherboard and a power cable that goes to the CD drive.

<img src="/pc98/cx/cx_psu.jpg" style="max-width:80%; padding-left: 10%" alt="PC-9821Cx PSU" title="PC-9821Cx PSU"/>

## 8. Remove the floppy drive cage.

The floppy drive cage has 5 screws that need to be removed. Four screws are attached to the cage. One screw is attached to the volume control and infrared sensor PCB on the front of the floppy drive cage.

<img src="/pc98/cx/cx_fdd_cage.jpg" style="max-width:80%; padding-left: 10%" alt="PC-9821Cx floppy drive cage" title="PC-9821Cx floppy drive cage"/>

Then pull the floppy drive cage up and out of the case, ensuring the power connector on the motherboard is disconnected.

## 9. Remove the CD drive cage.

Disconnect the data cable going to the CD drive.

The CD drive cage is held with 4 screws. Two of these screws on the back of the CD drive cage are attached to the motherboard. The other two screws at the front and to the side of the CD drive cage.

TODO: Image of screw locations
