<p align="center">
A simple example of more keyboard-centric Mac GUI buttons
</p>

<p align="center">  
</p>


## Introduction

Each button has its "accelerator" shortcut keys:
  - Highlighted with a custom font type/size/color and a unicode combining underscore when it appears inside the button label: __O̲__ verwrite
  - Added to button corners like <kbd>↩</kbd>

[Screenshot](./doc/Screenshot.png)

Also for rudimentary input debugging the app prints to console <kbd>↓</kbd>/<kbd>↑</kbd> key events and translates modifier changes (which do NOT generate <kbd>↓</kbd>/<kbd>↑</kbd> and due to this can't capture <kbd>⇪</kbd>CapsLock key presses properly, only key downs), e.g., pressing left then right <kbd>⇧</kbd>s (and releasing right then left) will notice the change in state and print the following:
```
Δ in ⇪‹⇧›‹⌃›‹⌥›‹⌘›🌐𝚻FlagsChanged vk=56 mod_flag=‹⇧
Δ in ⇪‹⇧›‹⌃›‹⌥›‹⌘›🌐𝚻FlagsChanged vk=60 mod_flag=‹⇧›
Δ in ⇪‹⇧›‹⌃›‹⌥›‹⌘›🌐𝚻FlagsChanged vk=60 mod_flag=‹⇧
Δ in ⇪‹⇧›‹⌃›‹⌥›‹⌘›🌐𝚻FlagsChanged vk=56 mod_flag=
```
Using `vk`s and  `cacao::appkit::EventModifierBitFlag` can help identify which modifier was pressed/released (except for <kbd>⇪</kbd>)

## Install

## Use

## Known issues

- Using accelerators that aren't "natively" attached to the buttons via `set_key_equivalent` doesn't generate the visual button press effect
- <kbd>⇪</kbd>CapsLock key↓↑ can't be tracked likely due to fundamental deficiencies of the underlying Mac key event mechanism

## Credits
