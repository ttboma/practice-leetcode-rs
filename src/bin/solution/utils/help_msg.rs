pub const FILE_HELP_MSG: &str =
    "A file from which input is read; if missing, input is read from stdin.";

pub const SUBCOMMAND_HELP_TEMPLATE: &str = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{tab}For examples,

{tab}```bash
{tab}$ solution fib
{tab}2
{tab}1
{tab}```

or

{tab}```bash
{tab}$ echo '2' | solution fib
{tab}1
{tab}```

{tab}Please refer to each of the following command and its associated LeetCode link for information on the input format.

{all-args}{after-help}
";
