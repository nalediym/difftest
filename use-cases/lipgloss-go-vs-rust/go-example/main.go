package main

import (
	"fmt"
	"os"
	"strings"

	"github.com/charmbracelet/lipgloss"
)

func main() {
	name := "World"
	if len(os.Args) > 1 && os.Args[1] != "" {
		name = os.Args[1]
	}

	// Header style: bold, colored, with rounded border
	headerStyle := lipgloss.NewStyle().
		Bold(true).
		Foreground(lipgloss.Color("205")).
		Background(lipgloss.Color("235")).
		PaddingLeft(2).
		PaddingRight(2).
		Border(lipgloss.RoundedBorder()).
		BorderForeground(lipgloss.Color("63")).
		Width(40).
		Align(lipgloss.Center)

	// Row label style
	labelStyle := lipgloss.NewStyle().
		Bold(true).
		Foreground(lipgloss.Color("86")).
		Width(12).
		Align(lipgloss.Right)

	// Row value style
	valueStyle := lipgloss.NewStyle().
		Foreground(lipgloss.Color("252"))

	// Print header
	greeting := fmt.Sprintf("Hello, %s!", name)
	fmt.Println(headerStyle.Render(greeting))
	fmt.Println()

	// Print info table rows
	rows := []struct {
		label string
		value string
	}{
		{"Name", name},
		{"Language", "Go"},
		{"Framework", "lipgloss"},
	}

	for _, row := range rows {
		label := labelStyle.Render(row.label + ":")
		value := valueStyle.Render(row.value)
		fmt.Println(lipgloss.JoinHorizontal(lipgloss.Top, label, " ", value))
	}

	// Footer separator
	fmt.Println(strings.Repeat("─", 40))
}
