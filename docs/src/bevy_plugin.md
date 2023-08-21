# Bevy Plugin

While Yarn Slinger is built to be engine-agnostic, the intended way to use it 
is through an engine-specific wrapper. The currently only supported engine is [Bevy](https://bevyengine.org/).
It is a data-oriented game engine using an ECS, which broadly means that you don't look at your game world
through the traditional lens of objects mutating the world and each other, but instead see the game as a collection
of data attached to various entities that can be queried and manipulated through systems.

This chapter will assume that you are familiar with the basics of Bevy. If you're not there not,
try to come back after you've gone through the [Bevy Book](https://bevyengine.org/learn/book/introduction/).
