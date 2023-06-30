+++
title = "CPU accelerators"
template = "page.html"
+++

The PC-9821Cx comes with a 486SX 33MHz soldered on the motherboard. Looking at the motherboard it would appear the CPU is not upgradable:

<!-- TODO: Shortcode for this image -->
<img src="/pc98/cx/cx_motherboard_a00.jpg" style="max-width:80%; padding-left: 10%" alt="PC-9821Cx Motherboard Revision A00" title="PC-9821Cx Motherboard Revision A00"/>

But the CPU in the PC-9821Cx can be upgraded with an additional accelerator card.

The accelerator card is installed in the 140-pin black connector shown below:

<img src="/pc98/cx/cx_motherboard_cpu_accel.jpg" style="max-width:80%; padding-left: 10%" alt="PC-9821Cx Motherboard CPU accelerator connector" title="PC-9821Cx Motherboard CPU accelerator connector"/>

Connector on the CPU accelerator card

<img src="/pc98/cx/cx_accelerator_connector_bottom.jpg" style="max-width:80%; padding-left: 10%" alt="PC-9821Cx CPU accelerator connector" title="PC-9821Cx CPU accelerator connector"/>

There are two options for an accelerator card: NEC PC-9821Cx-E01 and the Buffalo HCX-P33-AA.

Note that the PC-9821Cx provides 5 volts to the CPU, so if you wanted to use something that expects 3.3V your accelerator card would need a voltage regulator or you would need something like a [SocketBlaster].

## NEC PC-9821CX-E01

This is the official accelerator card for the PC-9821Cx.

TODO: Describe things about the card

TODO: Picture

## Buffalo HCX-P33-AA

This is an accelerator card that was sold by Melco/Buffalo.

TODO: Describe things about the card

TODO: Picture

[SocketBlaster]: https://github.com/scrapcomputing/486SocketBlaster

## Compatible CPUs

Since the PC-9821Cx uses a 486, the CPU would need to be 486 compatible. Generally this means up to a DX2-66, A DX4-100 if a voltage regulator is present or one of the third party 486 compatible CPUs (AMD and Cyrix for example).

If you mod your accelerator card it is apparently possible to put a [Pentium OverDrive in a PC-9821Cx shown in a tweet by Ethy1ene](https://twitter.com/Ethy1ene_L/status/1510300510321987587).

I have contacted Ethy1ene on discord to get more information (and will update when I get a response), but it appears the following was done to his CPU accelerator board:

1. An outer row of pins on the cpu was soldered to the CPU socket. I believe this is the case since there is flux around the outer row of pins and every CPU accelerator I've seen sold on Yahoo Auctions Japan has 3 rows of pins.
2. Two mods were done, one bridging two pins another involving a resistor (TODO: put the value?) between two pins. The tweet describes these mods.
3. Capacitor C4 is installed.
4. JP1, JP2 and JP4 are bridged together with a wire. I assume this works around the voltage regular and provides 5 volts directly to the CPU.
5. The voltage regular is not installed.
