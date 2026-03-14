Output demo/fail.gif

Set Shell "bash"
Set FontSize 16
Set Width 900
Set Height 550
Set Theme "Catppuccin Mocha"
Set Padding 20
Set TypingSpeed 50ms

Type "# Someone rewrote greet.sh — did they get it right?"
Enter
Sleep 1s

Type "diff tests/fixtures/greet.sh tests/fixtures/greet_broken.sh"
Enter
Sleep 2s

Type "# Let's find out with difftest"
Enter
Sleep 1s

Type "difftest tests/fixtures/greet.sh tests/fixtures/greet_broken.sh --inputs Naledi World"
Enter
Sleep 4s
