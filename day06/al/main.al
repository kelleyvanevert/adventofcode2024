// al run day06/al/main.al < inputs/input_06.txt

let example = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"

fn solve(input: str) {
  let grid = input :trim :lines :map chars
  let h = grid:len
  let w = grid[0]:len

  let (x, y) = grid
    :enumerate
    :find_map |(y, row)| {
      row :enumerate :find_map |(x, c)| {
        if c == "^" {
          (x, y)
        }
      }
    }

  let (dx, dy) = (0, -1)
  let seen = grid :map |row| { row :map |_| { 0 } }

  loop {
    //print("{x},{y} ({dx},{dy})")
    let (nx, ny) = (x + dx, y + dy)
    if nx < 0 || nx >= w || ny < 0 || ny >= h {
      //print("  off map")
      break
    }

    if grid[ny][nx] == "#" {
      (dx, dy) = (0 - dy, dx) :clone
      // !! `clone` is necessary due to a bug in AL that's been in there for a long time, maybe even since the beginning...
      //print("  make turn -> ({dx}, {dy})")
    } else {
      (x, y) = (nx, ny)
      //print("  walk -> {x},{y}")
      seen[y][x] = 1
    }
  }

  seen :map sum :sum
}

fn bonus(input: str) {
  let base_grid = input :trim :lines :map chars
  let h = base_grid:len
  let w = base_grid[0]:len

  let start = base_grid
    :enumerate
    :find_map |(y, row)| {
      row :enumerate :find_map |(x, c)| {
        if c == "^" {
          (x, y)
        }
      }
    }

  let num_loops = 0

  fn hash(x,y,dx,dy) {
    x * 100000 + y * 100 + (dx + 1) * 10 + (dy + 1)
  }

  for let oy in range(0, h) {
    'find: for let ox in range(0, w) {
      if base_grid[oy][ox] != "." {
        continue 'find
      }

      print("trying pos {ox},{oy} / {w},{h} ... {num_loops} loops found so far")
      let grid = base_grid :clone
      grid[oy][ox] = "#"

      let (x, y) = start :clone
      // !! `clone` is necessary due to a bug in AL that's been in there for a long time, maybe even since the beginning...

      let (dx, dy) = (0, -1)

      let been = [ hash(x,y,dx,dy) ]

      loop {
        let (nx, ny) = (x + dx, y + dy)
        if nx < 0 || nx >= w || ny < 0 || ny >= h {
          continue 'find
        }

        if grid[ny][nx] == "#" {
          (dx, dy) = (0 - dy, dx) :clone
          // !! `clone` is necessary due to a bug in AL that's been in there for a long time, maybe even since the beginning...
        } else {
          (x, y) = (nx, ny)
        }

        let ha = hash(x,y,dx,dy)
        if (ha :in been) {
          num_loops += 1
          continue 'find
        }

        been []= ha
        // !! `clone` is necessary due to a bug in AL that's been in there for a long time, maybe even since the beginning...
      }
    }
  }

  num_loops
}

print("Example solution: {solve(example)}")

// ±30ms
print("Solution: {solve(stdin)}")

print("Example bonus: {bonus(example)}")

// won't complete -- out of memory and waay too slow
//print("Bonus: {bonus(stdin)}")
