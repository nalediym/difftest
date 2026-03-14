Output demo/smoke.gif

Set Shell "bash"
Set FontSize 16
Set Width 900
Set Height 600
Set Theme "Catppuccin Mocha"
Set Padding 20
Set TypingSpeed 50ms

Type "# No inputs? difftest auto-generates smoke tests"
Enter
Sleep 1s

Type "difftest tests/fixtures/greet.sh tests/fixtures/greet.sh"
Enter
Sleep 4s

Type "# 7 auto-generated test cases. Zero config."
Enter
Sleep 2s
