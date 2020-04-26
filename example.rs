// def render(elt):
//         type_ = elt.type
//         attr = DOMMarkwdown.render_attrs(elt.attr) if elt.attr else ''
//         children = DOMMarkwdown.render_children(
//             elt.children) if elt.children else ''

//         if type_ == 'fragment':
//             return children

//         if type_ in VOID_ELEMENTS:
//             return '<%s%s/>' % (type_, attr)

//         if type_ == 'escaped_html':
//             return elt.markup

//         return '<%s%s>%s</%s>' % (type_, attr, children, type_)

fn main() {
    println!("Exporting!");
    let x: i32 = 100_000;
    println!("{}", i32::max_value());
    if x == 10 {
      println!("Ten!");
    } else {
      println!("Not ten!");
    }
  }
