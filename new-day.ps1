$day = "day" + ((Get-ChildItem -Path . -Name -Exclude *.*  | Measure-Object –Word).Words + 1)
cargo new $day && cd $day
