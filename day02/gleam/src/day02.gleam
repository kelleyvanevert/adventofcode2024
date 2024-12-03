import gleam/function
import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  // let assert Ok(files_and_folders) = simplifile.read_directory(at: "../../inputs")
  // io.debug(files_and_folders)

  let assert Ok(input) = simplifile.read(from: "../../inputs/input_02.txt")

  let reports =
    input
    |> string.trim
    |> string.split("\n")
    |> list.map(fn(str) {
      str
      |> string.split(" ")
      |> list.map(int.parse)
      |> result.values
    })

  let safe = list.filter(reports, is_report_safe)

  io.println("Safe:")
  io.debug(list.length(safe))

  let bonus_safe =
    reports
    |> list.map(report_variations)
    |> list.filter(list.any(_, is_report_safe))

  io.println("\nBonus safe:")
  io.debug(list.length(bonus_safe))
}

fn is_report_safe(levels: List(Int)) -> Bool {
  {
    levels
    |> list.window_by_2
    |> list.map(fn(window) {
      let #(a, b) = window
      a > b && a - b >= 1 && a - b <= 3
    })
    |> list.all(function.identity)
  }
  || {
    levels
    |> list.window_by_2
    |> list.map(fn(window) {
      let #(a, b) = window
      a < b && b - a >= 1 && b - a <= 3
    })
    |> list.all(function.identity)
  }
}

fn report_variations(levels: List(Int)) -> List(List(Int)) {
  [
    levels,
    ..list.index_map(levels, fn(_, i) {
      let #(left, right) = list.split(levels, i)
      list.append(left, list.drop(right, 1))
    })
  ]
}
