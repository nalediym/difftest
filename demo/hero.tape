Output demo/hero.gif

Set Shell "bash"
Set FontSize 16
Set Width 900
Set Height 650
Set Theme "Catppuccin Mocha"
Set Padding 20
Set TypingSpeed 40ms

Type "# You rewrote greet.sh. Does the rewrite still work?"
Enter
Sleep 1.5s

Type "difftest tests/fixtures/greet.sh tests/fixtures/greet_broken.sh --inputs Naledi World"
Enter
Sleep 4s

Type ""
Enter
Type "# It caught the bug. 'Hello' was changed to 'Hi'."
Enter
Sleep 2s

Type "# Now test the fixed version:"
Enter
Sleep 1s

Type "difftest tests/fixtures/greet.sh tests/fixtures/greet.sh --inputs Naledi World"
Enter
Sleep 4s
