# FRCLib

A library to facilitate the use of rust in frc in a rusty way.

There have been many prior projects to achieve this but most simply wrapped wpilib C++ apis, thats no fun.
The goal of this project is to make *the entire stack* rust, not because it gets you more points but purely to learn and have fun.
A core concept behind this library is the plugability.
Is the frclib rio hal isn't working or being maintained? you can switch to another hal in 1 line of user code.

This library is very opinionated in certain ways, so much so I am souring on some of my original decisions (the use of linkme).

I will have a much larger writeup and architecture goals/docs soon if you stumble across this now.
