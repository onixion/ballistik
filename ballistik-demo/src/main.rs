use ballistik::Ballistik;
use ballistik::views::test::TestView;

fn main() {

    let mut ballistik = Ballistik::new();

    ballistik.add_view(Box::new(TestView::default()));

    ballistik.run();
}
