
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'artem' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'artem'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'artem' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'Change the characters that are used to display the image.The first character should have the highest ''density'' and the last should have the least (probably a space).A lower detail map is recommend for smaller images.')
            [CompletionResult]::new('--characters', 'characters', [CompletionResultType]::ParameterName, 'Change the characters that are used to display the image.The first character should have the highest ''density'' and the last should have the least (probably a space).A lower detail map is recommend for smaller images.')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'Change the size of the output image.The minimum size is 20, the maximum 230. Values outside of the range will beignored and changed to the nearest usable value. This argument is conflicting with --width and --height')
            [CompletionResult]::new('--size', 'size', [CompletionResultType]::ParameterName, 'Change the size of the output image.The minimum size is 20, the maximum 230. Values outside of the range will beignored and changed to the nearest usable value. This argument is conflicting with --width and --height')
            [CompletionResult]::new('--ratio', 'ratio', [CompletionResultType]::ParameterName, 'Change the ratio between height and width, since Ascii chars are a bit higher than long.The default value is 0.43, min is 0 and max 2. It is not recommend to change this setting.')
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Output file for non-colored ascii. If the output file is a plaintext file, no color will be used. The use color, either use a file with an .ansi extension, or an .html file, to convert the output to html.')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'Output file for non-colored ascii. If the output file is a plaintext file, no color will be used. The use color, either use a file with an .ansi extension, or an .html file, to convert the output to html.')
            [CompletionResult]::new('--thread', 'thread', [CompletionResultType]::ParameterName, 'OutputNumber of threads used to convert the image. A larger number can lead to grater performance. Defaults to 4')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Choose the verbosity of the logging level.')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Use the terminal maximum terminal height to display the image.This argument is conflicting with --size and --width ')
            [CompletionResult]::new('--height', 'height', [CompletionResultType]::ParameterName, 'Use the terminal maximum terminal height to display the image.This argument is conflicting with --size and --width ')
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'Use the terminal maximum terminal height to display the image.This argument is conflicting with --size and --height ')
            [CompletionResult]::new('--width', 'width', [CompletionResultType]::ParameterName, 'Use the terminal maximum terminal height to display the image.This argument is conflicting with --size and --height ')
            [CompletionResult]::new('--flipX', 'flipX', [CompletionResultType]::ParameterName, 'Flip the image along the X axis')
            [CompletionResult]::new('--flipY', 'flipY', [CompletionResultType]::ParameterName, 'Flip the image along the Y axis')
            [CompletionResult]::new('--invert', 'invert', [CompletionResultType]::ParameterName, 'Inverts the characters used for the image, so light characters will as dark ones. Can be useful if the image has a dark background.')
            [CompletionResult]::new('--background', 'background', [CompletionResultType]::ParameterName, 'Sets the background of the ascii as the color. This will be ignored if the terminal does not support truecolor. This argument is mutually exclusive with the no-color argument.')
            [CompletionResult]::new('--border', 'border', [CompletionResultType]::ParameterName, 'Adds a decorative border surrounding the ascii image. This will make the image overall a bit smaller, since it respects the user given size.')
            [CompletionResult]::new('--no-color', 'no-color', [CompletionResultType]::ParameterName, 'Do not use color when printing the image to the terminal.')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
