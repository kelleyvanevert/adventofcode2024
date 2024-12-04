// al run day04/al/main.al < inputs/input_04.txt

// If there's a struct `bla`, then `<bla>` is just a way of creating an instance,
//  and `<bla class="bla">` additionally sets the attribute `.bla` (if exists and correctly typed),
//  and `<bla class("bla">` additionally calls the method `.bla(..)` (if exists and correctly typed),
//  and `<bla>{ child }` additionally calls the method `.add` (if exists and correctly typed)

// You can also define methods/setters explicitly like so:
//
// struct div {
//   .class: string
// 
//   .class: string set |self, new| { }
// }
// 
// fn div.class=(self, name: str) {
//   self.name = name
// }

// trait html_element {
//   fn add(child: html_element): nil
// }

let b = div {}

fn header(c, p) {
  <div
    class("mx-4 bg-blue hover:bg-red")
  >{
    <h1>{ p.title }
    <row>{
      <p.child>{
        <p.row()>{

        }
      }
    }
  }
}

fn app(c, p) {
  <div>{
    header(c)
  }
}
