Output demo/pass.gif

Set Shell "bash"
Set FontSize 16
Set Width 900
Set Height 500
Set Theme "Catppuccin Mocha"
Set Padding 20
Set TypingSpeed 50ms

Type "# Two identical programs — do they behave the same?"
Enter
Sleep 1s

Type "cat tests/fixtures/greet.sh"
Enter
Sleep 2s

Type "# Let's test with difftest"
Enter
Sleep 1s

Type "difftest tests/fixtures/greet.sh tests/fixtures/greet.sh --inputs Naledi World ''"
Enter
Sleep 4s
