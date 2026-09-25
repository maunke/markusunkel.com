---
title: From QWERTZ to custom splitted Colemak Mod-DH
description: My experience with the transition from QWERTZ to custom splitted Colemak Mod-DH on MOONLANDER MARK I keyboard.
date: 2024-12-14
---

I have always had a QWERTZ keyboard (german version of QWERTY; swapped y with z and some keys for
umlauts), but in the summer of 2022 I found myself in that typical 'let me change a few things that
bother me' phase. One of those things was the hand pain I experienced after a full day of coding.
After entering the rabbit hole of ergonomic keyboards, I discovered the universe of alternative
keyboard layouts for the first time ..

## Origin of QWERTY

.. and was astonished, that the one layout, I have been used since always for everything on
computers, smartphones and displays, is originated in an **improvement in type-writing machines**
[patented in Aug. 27, 1878](https://patents.google.com/patent/US207559A), see below figure 3 of the
patent.

![QWERTY layout in [US Patent 207559](//patents.google.com/patent/US207559A) figure 3.](patent-us-207559-fig-3.webp "QWERTY layout in US patent 207559 figure 3.")

In that patent no single word is wasted to explain the design decisions behind the presented
keyboard layout. After reading the paper
[On the Prehistory of QWERTY](//repository.kulib.kyoto-u.ac.jp/dspace/bitstream/2433/139379/1/42_161.pdf)
of Koichi Yasuoka and Motoko Yasuoka with its conclusion

> We have presented that Type-Writer keyboard was originally derived from Hughes-Phelps Printing
> Telegraph, and that QWERTY was developed for Morse receivers. The development of QWERTY was a
> winding road, first by Sholes and others, second by Harrington and Craig, then by Jenne and
> Clough, again by Sholes, and at last by Wyckoff, Seamans & Benedict. There was no consistent
> policy towards QWERTY. The keyboard arrangement was incidentally changed into QWERTY, first to
> receive telegraphs, then to thrash out a compromise between inventors and producers, and at last
> to evade old patents.
>
> -- Part of conclusion in
> [On the Prehistory of QWERTY](//repository.kulib.kyoto-u.ac.jp/dspace/bitstream/2433/139379/1/42_161.pdf)

it makes the whole topic more astonishing and more or less random in the context of using QWERTZ as
keyboard layout in 21st century. Is QWERTY just one very old technical debt in today's digital
interface design?

## Keyboard layout metrics

Before choosing a new keyboard layout I searched for some metrics to compare keyboard layouts and
found some interesting ones (see
[here some literature](//docs.google.com/document/d/1Ic-h8UxGe5-Q0bPuYNgE3NoWiI8ekeadvSQ5YysrwII))
to consider:

Same finger bigrams (SFBs)
: A SFB consists of pressing two keys in succession with the same finger.

Same finger Skipgrams (SFSs)
: A SFS consists of pressing two keys with the same finger, but separated by X letters.

Lateral stretch bigrams (LSBs)
: A bigram that pulls two of our fingers apart or that forces us to laterally shift our wrist a bit
  to go from one key to the other.

Alt fingering
: Using a finger other than the intended one to type a certain bigram, with the purpose of avoiding
  a SFB, is referred to alt fingering.

SFB collisions
: A collision refers to when alt fingering a SFB creates a new SFB.

Full scissors bigrams (FSBs)
: The vertical separation between the keys is two rows. The finger that prefers being higher is not.

Roll
: Pressing two keys with one hand, and a third key with the other.

Redirect
: An one-handed trigram in which the direction changes.

### QWERTY vs Dvorak vs Colemak Mod-DH

In the following table you can see a comparison between QWERTY and two preselected mainstream
alternative layouts Dvorak and Colemak Mod-DH, based on english grammar. The metrics are calculated
by [Layout Playground](//oxey.dev/playground/).

| Metric           | QWERTY | Dvorak     | Colemak Mod-DH |
| ---------------- | ------ | ---------- | -------------- |
| Home keys usage  | 25.26% | 56.70%     | **62.31%**     |
| SFB              | 6.60%  | 2.78%      | **1.37%**      |
| LSB              | 6.88%  | **1.26%**  | 1.98%          |
| Total Rolls      | 37.24% | 38.92%     | **46.11%**     |
| Total Alternates | 26.48% | **44.56%** | 30.38%         |
| Total Redirects  | 13.19% | **3.458%** | 10.58%         |

Table: Comparison of selected three main layouts by efficiency and ergonomics metrics.

The most weighted metrics for me are **Home keys usage**, **SFB** and **LSB**, such that **Colemak
Mod-DH** was the winner of the first iteration. In the following figures you can see the Colemak
Mod-DH colorized by the key usage for the english grammar (red means high usage) and in contrast to
the QWERTY layout key usage.

![Colemak DH layout key usage heatmap for the english grammar.](colemak-dh-metrics.svg "Colemak DH layout key usage heatmap for the english grammar.")

![QWERTY layout key usage heatmap for the english grammar.](qwerty-metrics.svg "QWERTY layout key usage heatmap for the english grammar.")

It is immediately noticeable how the weight of the key usage falls more in the middle row (home
keys) in case of the Colemak Mod-DH layout.

In the this section I label the selection of Colemak Mod-DH as the first iteration because we have
to iterate two more times.

## Ergonomic keyboard

As described above the hand pain was the initial trigger for my investigation of a new keyboard
layout. Beside the layout I started to search for a suitable keyboard, that

Req. A.
: allows to embed the Colemak Mod-DH layout incl. the final layout with special characters.

Req. B.
: brings the hand into a position where the hand tendors do not overstretch.

Req. C.
: lets my chest remain open (e.g. a very small keyboard in the center in front of me would narrow my
  chest).

Req. D.
: is programmable and translates the keyboard signals on device instead of moving that part to my
  computer.

The requirements **B** and **C** imply an angled and splitted keyboard. A short research to fulfill
requirements **A** and **D** brought me to the keyboard manufacturer [ZSA](//zsa.io). Fortunately
all their keyboards fulfill my requirements **B** to **D**.

After reading many reviews of the ZSA keyboards on reddit I started to choose one of their keyboards
and selected the brand new [MOONLANDER MARK I](//www.zsa.io/moonlander), but before buying I needed
to create my own layered layout that I can work with in order to fulfill requirement **A**.

![[ZSA Moonlander](//www.zsa.io/moonlander) splitted keyboard.](moonlander.webp "ZSA Moonlander splitted keyboard.")

The MOONLANDER keyboard can even be brought into an even more angled position withelp of the
[ZSA Platform](//www.zsa.io/moonlander/platform).

![[Platform](//www.zsa.io/moonlander/platform) (tenting system) for ZSA Moonlander splitted keyboard.](moonlander-platform.webp "Platform (tenting system) for ZSA Moonlander splitted keyboard.")

In the next section we will see the creation of the custom layout based on the MOONLANDER MARK I.

## Splitted keyboard with custom Colemak Mod-DH

With the help of the [ORYX keyboard configurator](//configure.zsa.io/home) I started to implement
the Colemak Mod-DH layer incl. german umlauts ä, ö, ß and ü. I decided to design the implementation
of a second character on keys by holding the key for longer than 200ms. I like the kind of rythm
implied by that design in german words like "Beste Grüße" (best regards), that are used at the end
of german emails or letters.

In the same way I started to design the symbols layer such that the complementary symbol is 200ms
away from the original symbol, e.g. tap for "[" but hold for "]", or tap "?" and hold "!". In the
most cases I don't need to hold symbol keys since I am using auto close completion in Neovim. In
cases like "?" and "!" the "?" character is used more often due to the ? Operator in Rust.

Please find [my latest keyboard layout](//configure.zsa.io/moonlander/layouts/NJPQB/latest) on the
configuration page of ORYX. Alternatively you can have a look at the following two subsections that
show the Colemak Mod-DH and symbols layer.

### Colemak Mod-DH layer

![Left side of splitted custom Colemak Mod-DH layer.](colemak-left.webp "Left side of splitted custom Colemak Mod-DH layer.")

![Right side of splitted custom Colemak Mod-DH layer.](colemak-right.webp "Reft side of splitted custom Colemak Mod-DH layer.")

### Symbols layer

By holding the layer 1 key above I can access all symbols keys below.

![Left side on symbols layer of splitted custom Colemak Mod-DH.](symbols-left.webp "Left side on symbols layer of splitted custom Colemak Mod-DH.")

![Right side on symbols layer of splitted custom Colemak Mod-DH.](symbols-right.webp "Right side on symbols layer of splitted custom Colemak Mod-DH.")

### Heatmap

While using the [heatmap tracker of ORYX](//configure.zsa.io/train/heatmap) for some minutes I wrote
some Rust and Python code, a german and an english email. As you can see in the heatmap below (from
blue to red, from low to high usage) I am quite satisfied about the home key usage. As expected, the
space key is the winner with the most tap events, followed by "E" and the symbols layer key. So my
right thumb is quite in usage, but I never felt a hand pain on the new keyboard layer since I am
using it.

![Key usage heatmap of my custom splitted Colemak DH (low [blue] to high [red] usage).](heatmap-layout.webp "Key usage heatmap of my custom splitted Colemak DH (low [blue] to high [red] usage).")

### Laptop

After having an idea how to switch from QWERTZ to the new layout I tackled the challenge of bringing
that to my laptop. A short search for a keyboard manager resulted in
[kmonad](//github.com/kmonad/kmonad), that enables me to use layers, tap and hold functionalities
exactly the same way I defined the keys with ORYX on the splitted keyboard. I switched the keys on
the laptop keyboard to the Colemak Mod-DH layer and moved the remaining buttons to the positions
where shift, enter, tab, delete and other keys are located. Please see result of that below.

Some hours later I wrote the final
[kmonad config](//paste.sr.ht/~maunke/5728ac3190c3900519a2c622f6ebdf01c78f7b15), that I am still
using today. The transition was completed in that moment.

![Laptop keyboard with flattened custom splitted Colemak DH layout (a scan of a physical Instax Wide Evo print).](laptop-keyboard.webp "Laptop keyboard with flattened custom splitted Colemak DH layout (a scan of a physical Instax Wide Evo print).")

## Conclusion

More than two years after switching from QWERTZ to the custom splitted Colemak Mod-DH I never felt
hand pain again and I am quite happy about it. Interestingly enough I discovered for my first time
muscle memory, that hitted me in unexpected moments for around 6 months after switching.

![Splitted keyboard with reMarkable paper pro in portrait mode (a scan of a physical Instax Wide Evo print).](keyboard.webp "Splitted keyboard with reMarkable paper pro in portrait mode (it is a scan of a physical Instax Wide Evo print).")

A very nice side effect of using a splitted keyboard is to put something to write on between in
portrait mode, such that I am writing on my
[reMarkable tablet](//remarkable.com/store/remarkable-paper/pro) during programming, ideation or
meeting sessions.

Update September 18th, 2026
: This summer I switched to the ZSA Voyager keyboard with a new home key focused
  [Colemak DH layout](//configure.zsa.io/voyager/layouts/Drv57).
