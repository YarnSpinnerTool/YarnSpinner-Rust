# Basics

Yarn Spinner stores its dialogue in *Yarn files*. These look a bit like play scripts or movie scripts.
Here is an example that might be called `hello_world.yarn`:

```text
title: Start
---
Hello World!
===
```

Simple, right? Let's go through it line-by-line.

- Yarn files are organized in *nodes*. The first line in this script, `title: Start`, 
tells us that this is the start of the node named `Start`. We call this kind of information a `header`.
- The line `---` separates the headers from the *body*.
- `Hello World` is the text that will be shown to the player.
- `===` marks the end of the node.

And that's the whole file! If you run this node, you will get a dialogue box with the text "Hello World!".

In the next chapter, we will look at how you can run examples along.
