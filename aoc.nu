#!/bin/nu
const WORKDIR = ("~/Projects/AdventOfCode/" | path expand)
const INPUT_URL = "https://adventofcode.com/{year}/day/{day}/input"

def RESET_TIME []  { (date now) + 1day | format date '%Y-%m-%d 00:00:00 -5' | into datetime }
def DEC_1ST []  { date now | format date '%Y-12-01' | into datetime }

def input_url [input: int, day: int, year: int] {
    let data = { 
            year: ($"{input}")
            day: ( $day | fill -a r -c '0' -w 2)
            n: $input
        }
    $data | format $INPUT_URL
}

def generate_day [day?: int, year?: int] {
    let current_day = if $day == null {
        date now | format date "%d" | into int
    } else {$day}
    let current_year = if $year == null {
        date now | format date "%Y"
    } else {$year}
    let path = $"year-($current_year)/day-($current_day | fill -a r -c '0' -w 2)"
    cargo generate --path $"($WORKDIR)/daily-template" -f --name $'"($path)"'
}

def wait_next_day [] {
    let current = date now
    let dec_1st = $current | format date '%Y-12-01' | into datetime
    let dt = $dec_1st - $current
    let wait = if $dt > 0sec {
        # Not the month
        $dt
    } else {
        if RESET_TIME > ($dec_1st + 24day) {
            print "Wait till next year!"
            exit 0
        }
        RESET_TIME
    }
    print $'Sleeping for ($wait)...'
    sleep $wait
}

def day_exists [workdir: path, day?: int] -> boolean {
    let current_day = if $day == null {
        date now | format date "%d"
    } else {$day | fill -a r -c '0' -w 2}

    let current_year = date now | format date "%Y" | into int
    if (date now) > (DEC_1ST) and (date now) > ((DEC_1ST) + 24day) {
        return ($workdir | path join $"year-($current_year)/day-($current_day)" | path exists)
    }
    return true # days outside of advent shouldn't be created
}

export def "aoc generate" [day: int, year?: int] {
    generate_day $day $year
}

export def "aoc day" [day: int] {
    if (day_exists $WORKDIR) {
        # Current day exists! Waiting for next day...
        wait_next_day
    }
    generate_day
}

export def "aoc input" [
        day: int, 
        input: int, 
        old_year?: int
    ] {
    let year = if $old_year == null {
        date now | format date "%Y"
    } else {$old_year}
    let path = $"year-($year)/day-($day | fill -a r -c '0' -w 2)"
    wget --load-cookies ($WORKDIR | path join "cookies.txt") (input_url 1 $day $year) -o ($WORKDIR | path join $path $"input($input).txt")
}