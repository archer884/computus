Daily Programmer #202: Computus
===============================

This intermediate challenge is a little mathier than the average, but that's not as biga  deal as one might
imagine because of the fact that algorithms for figuring out when Easter Sunday falls are literally hundreds
of years old and (obviously) well-publicized. My initial solution for this (written in C#) actually takes
advantage of a random personal website somewhere and `System.Net.WebClient` to load and parse the required
data without doing any actual calculations. It's amazing what you can achieve with two regular expressions,
some linq, and a little stubbornness.

This solution instead makes use of an [anonymous Gregorian algorithm](http://en.wikipedia.org/wiki/Computus#Anonymous_Gregorian_algorithm)
to achieve the intended result, calculating the month and day of Easter Sunday via some of the most arcane
mathematics imaginable.

Of course, the challenge output portion of today's assignment is a little bit more annoying to deal with in
bash/linux/etc., but--in the spirit of learning new things--I have written a bash script that carries out
the necessary calculations.
