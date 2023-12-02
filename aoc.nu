#!/bin/nu
const INPUT_URL = "https://adventofcode.com/{year}/day/{day}/input"
const DAILY_RESET_TIME = '%Y-%m-%d 00:00:00 -5'

const WORKDIR = ("~/Projects/AdventOfCode/" | path expand)
const TEMPLATE_DIR = ($WORKDIR | path join "templates/")
const COOKIE_PATH = ($WORKDIR | path join "cookies.txt")
const YEAR_FORMAT = "year-{year}"
const DAY_FORMAT = "day-{day}"

const RUST_CONFIG = {project_name: "rs-{day}", template_name: "rust-template"}
const NU_CONFIG = {project_name: "nu-{day}", template_name: "nu-template"}


def RESET_TIME [] -> date  { (date now) + 1day | format date $DAILY_RESET_TIME | into datetime }
def DEC_1ST [] -> date { date now | format date '%Y-12-01' | into datetime }

# Right pad an input string with character c
def pad [str, c: string, num: int] -> string {
    $str | fill -a r -c $c -w $num
}

# Get the path for a particular day
def get_day_path [year: int, day: int] -> path {
    let path = $WORKDIR | path join $YEAR_FORMAT $DAY_FORMAT
    {day: (pad $day '0' 2), year: (pad $year '0' 4)} | format $path
}

def get_project_template [config: record<project_name: string, template_name: string>] -> path {
    $TEMPLATE_DIR | path join $config.template_name
}

def get_project_path [config: record<project_name: string, template_name: string>, year: int, day: int] -> path {
    let path = $WORKDIR | path join (get_day_path $year $day) $config.project_name
    {day: (pad $day '0' 2), year: (pad $year '0' 4)} | format $path
}

def input_url [input: int, day: int, year: int] -> string {
    let data = {
            day: ($day)
            year: ($year)
            n: $input
        }
    $data | format $INPUT_URL
}

def with_cookies [url: string, output: path] {
    wget --load-cookies $COOKIE_PATH $url -O $output
}

def generate_rust_template [day: int, year: int] {
    let project_path = get_day_path $year $day
    if not ($project_path | path exists) {
        mkdir $project_path
    }
    cd $project_path
    let name = {year: (pad $year '0' 4), day: (pad $day '0' 2) } | format $RUST_CONFIG.project_name
    if ($"./($name)" | path exists) {
        print "Project already exists"
        return
    }
    cargo generate --path (get_project_template $RUST_CONFIG) -f --name $name
}


def delay_next_day [] {
    let current = date now
    let dt = (DEC_1ST ) - $current
    let wait = if $dt > 0sec {
        # Not the month
        $dt
    } else {
        if (RESET_TIME ) > ((DEC_1ST ) + 24day) {
            print "Wait till next year!"
            exit 0
        }
        (RESET_TIME) - $current
    }
    print $'Sleeping for ($wait)...'
    sleep $wait
}

export def "aoc generate rust" [day: int, year?: int] {
    let selected_day = if $day == null {
        date now | format date "%d" | into int
    } else {$day}
    let slected_year = if $year == null {
        date now | format date "%Y"
    } else {$year}
    let date = $"($slected_year)-12-($selected_day)" | into datetime -o -5
    print "Generating rust template..."
    generate_rust_template $selected_day $slected_year
    if (date now) < $date {
        mut wait_time  = $date - (date now)
        while $wait_time > 0sec {
            print $"Waiting for ($date)... ($wait_time) remaining..."
            let halving = $wait_time / 2
            if $halving > 1sec {
                sleep $halving
            } else {
                sleep $wait_time
            }
            $wait_time = $date - (date now)
        }
    }
    # The current day/year is available, obtain the inputs
    print "Obtaining inputs..."
    aoc input $selected_day 1 $slected_year
}

export def "aoc day" [day: int, year?: int] {
    if (day_exists $WORKDIR) {
        # Current day exists! Waiting for next day...
        delay_next_day
    }
    generate_day $day $year
}

export def "aoc input" [
        day: int, # Day to obtain input for
        input: int, # Input number, if multiple
        year?: int # Given year, defaults to current
    ] {
    let selected_year = if $year == null {
        date now | format date "%Y"
    } else {$year}
    let url = {year: $selected_year, day: $day} | format $INPUT_URL
    with_cookies $url (get_day_path $selected_year $day | path join $"input($input).txt")
}