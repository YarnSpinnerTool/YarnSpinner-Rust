# Nodes

In the last chapters, we have so far only used a single node named "Start".
We will now use multiple nodes and `jump` between them to compose a more complex dialogue:

```text
title: Start
---
Ferris: Say, do you want to go on an adventure?
-> Aye aye!
    Ferris: Great, let's go!
    <<jump Adventure>>
-> No thanks.
    Ferris: Okay, that's fine.
    <<jump GoodBye>>
===

title: Adventure
---
Narrator: And so, the two friends went on an adventure.
Dictionary: timeskip (pl. timeskips)(fandom slang): An instance of fast-forwarding a substantial amount of time, such as years or decades, as a narrative device in a story, quickly aging characters and developing events.
Ferris: Wow, that was a great adventure!
<<jump GoodBye>>
===

title: GoodBye
---
Narrator: And everyone lived happily ever after.
===
```

Here we've got three nodes: "Start", "Adventure", and "GoodBye". We jump between them using the `jump` command.
You can see that we always arrive at the node "GoodBye", but optionally go through the node "Adventure" first.
If you're editing your Yarn file using Visual Studio Code and have the Yarn Spinner extension installed,
you can display this flow in a graph by clicking the "Show Graph" button in the upper right corner, which will show you something like this:
![graph.png](graph.png)

It is allowed to jump to your current node:
```text
title: Start
---
Ferris: Say, do you want to go on an adventure?
-> Aye aye!
    Ferris: Great, let's go!
-> No thanks.
    Ferris: Pretty please?
    <<jump Start>>
===
```
