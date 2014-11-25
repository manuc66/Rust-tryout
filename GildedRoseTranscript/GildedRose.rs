fn main() {
	let mut items = vec!( Item { quality: 10, sell_in: 5, name : "Aged Brie" });
	
	update_quality(items.as_mut_slice());
	println!("quality: {}", items[0].quality)
	println!("sell_in: {}", items[0].sell_in)
	println!("name: {}", items[0].name)
}

struct Item {
    quality: int,
	sell_in: int,
	name: &'static str
}

fn update_quality(items: &mut[Item]) {
	for i in range(0, items.len()) {
        if items[i].name != "Aged Brie" && items[i].name != "Backstage passes to a TAFKAL80ETC concert"
        {
            if items[i].quality > 0
            {
                if items[i].name != "Sulfuras, Hand of Ragnaros"
                {
                    items[i].quality = items[i].quality - 1;
                }
            }
        }
        else
        {
            if items[i].quality < 50
            {
                items[i].quality = items[i].quality + 1;

                if items[i].name == "Backstage passes to a TAFKAL80ETC concert"
                {
                    if items[i].sell_in < 11
                    {
                        if items[i].quality < 50
                        {
                            items[i].quality = items[i].quality + 1;
                        }
                    }

                    if items[i].sell_in < 6
                    {
                        if items[i].quality < 50
                        {
                            items[i].quality = items[i].quality + 1;
                        }
                    }
                }
            }
        }

        if items[i].name != "Sulfuras, Hand of Ragnaros"
        {
            items[i].sell_in = items[i].sell_in - 1;
        }

        if items[i].sell_in < 0
        {
            if items[i].name != "Aged Brie"
            {
                if items[i].name != "Backstage passes to a TAFKAL80ETC concert"
                {
                    if items[i].quality > 0
                    {
                        if items[i].name != "Sulfuras, Hand of Ragnaros"
                        {
                            items[i].quality = items[i].quality - 1;
                        }
                    }
                }
                else
                {
                    items[i].quality = items[i].quality - items[i].quality;
                }
            }
            else
            {
                if items[i].quality < 50
                {
                    items[i].quality = items[i].quality + 1;
                }
            }
        }
    }
}