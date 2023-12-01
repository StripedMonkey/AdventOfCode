#!/bin/nu
const INPUT_URL = "https://adventofcode.com/{year}/day/{day}/input"
const DAILY_RESET_TIME = '%Y-%m-%d 00:00:00 -5'

const WORKDIR = ("~/Projects/AdventOfCode/" | path expand)
const COOKIE_PATH = ($WORKDIR | path join "cookies.txt")
const YEAR_FORMAT = "year-{year}"
const DAY_FORMAT = "day-{day}"

const RUST_CONFIG = {project_name: "rs-{day}", template_name: "daily-template"}
const NU_CONFIG = {project_name: "nu-{day}", template_name: "daily-template-nu"}


def RESET_TIME [] -> date  { (date now) + 1day | format date $DAILY_RESET_TIME | into datetime }
def DEC_1ST [] -> date { date now | format date '%Y-12-01' | into datetime }

# Right pad an input string with character c
def pad [c: string, num: int] -> string {
    $in | fill -a r -c $c -w $num
}

def get_day_path [year: int, day: int] -> path {
    let path = $WORKDIR | path join $YEAR_FORMAT $DAY_FORMAT
    {day: ($day | pad '0' 2), year: ($year | pad '0' 4)} | format $path
}

def get_project_template [config: record<project_name: string, template_name: string>] -> path {
    $WORKDIR | path join $config.template_name
}

def get_project_path [config: record<project_name: string, template_name: string>, year: int, day: int] -> path {
    let path = $WORKDIR | path join (get_day_path $year $day) $config.project_name
    {day: ($day | pad '0' 2), year: ($year | pad '0' 4)} | format $path
}

def input_url [input: int, day: int, year: int] -> string {
    let data = {
            day: ($day | into string)
            year: ($year | into string)
            n: $input
        }
    $data | format $INPUT_URL
}

def get_input [cookie_path: path, url: string, output: path] {
    wget --load-cookies $cookie_path $url -o $output
}

def generate_rust_template [year: int, day: int] {
    let project_path = get_day_path $year $day
    if not ($project_path | path exists) {
        mkdir $project_path
    }
    cd $project_path
    let name = {year: ($year | pad '0' 4), day: ($day | pad '0' 2) } | format $RUST_CONFIG.project_name
    cargo generate --path (get_project_template $RUST_CONFIG) -f --name $name
}

def rust_generate_day [day?: int, year?: int] {
    let current_day = if $day == null {
        date now | format date "%d" | into int
    } else {$day}
    let current_year = if $year == null {
        date now | format date "%Y"
    } else {$year}
    generate_rust_template $current_year $current_day
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
    rust_generate_day $day $year
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
    # wget --load-cookies ($WORKDIR | path join "cookies.txt") (input_url 1 $day $year) -o ($WORKDIR | path join $path $"input($input).txt")
}