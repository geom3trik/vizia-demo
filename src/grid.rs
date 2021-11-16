use std::{collections::HashSet, marker::PhantomData, rc::Rc};

use vizia::*;

pub struct Grid<L, T: 'static> 
where L: Lens<Target = Vec<T>>,
{
    columns: usize,
    rows: usize,
    lens: L,
    builder: Option<Rc<dyn Fn(&mut Context, usize, ItemPtr<L,T>)>>,
}

impl<L, T> Grid<L,T> 
where L: Lens<Target = Vec<T>>,
{
    pub fn new<F>(cx: &mut Context, columns: usize, rows: usize, lens: L, builder: F) -> Handle<Self> 
    where F: 'static + Fn(&mut Context, usize, ItemPtr<L,T>)
    {

        println!("New Grid: {}", columns);
        // More or less everything in here should be done internally by vizia

        let parent = cx.current;
        let list = Self {
            columns,
            rows,
            lens,
            builder: Some(Rc::new(builder)),
        };

        // Assign an id if one doesn't already exist but don't call `body`.
        let id = if let Some(id) = cx.tree.get_child(cx.current, cx.count) {
            cx.views.insert(id, Box::new(list));
            id
        } else {
            let id = cx.entity_manager.create();
            cx.tree.add(id, cx.current).expect("Failed to add to tree");
            cx.cache.add(id).expect("Failed to add to cache");
            cx.style.borrow_mut().add(id);
            cx.views.insert(id, Box::new(list));
            id  
        };

        cx.count += 1;

        // Add this view as an observer to the lensed model
        let mut ancestors = HashSet::new();
        for entity in parent.parent_iter(&cx.tree) {
            ancestors.insert(entity);

            if let Some(model_list) = cx.data.model_data.get_mut(entity) {
                for (_, model) in model_list.iter_mut() {
                    if let Some(store) = model.downcast::<Store<L::Source>>() {
                        if store.observers.intersection(&ancestors).next().is_some() {
                            break;
                        }
                        store.insert_observer(id);
                    }
                }
            }
        }

        // Now that the observer has been set up we can call the `body`
        if let Some(mut view_handler) = cx.views.remove(&id) {
            let prev = cx.current;
            cx.current = id;
            let prev_count = cx.count;
            cx.count = 0;
            view_handler.body(cx);
            cx.current = prev;
            cx.count = prev_count;
            cx.views.insert(id, view_handler);
        }
        
        // Create and return a handle to this view
        let handle = Handle {
            entity: id,
            style: cx.style.clone(),
            p: PhantomData::default(),
        };

        handle.layout_type(LayoutType::Grid)
        //.row_between(Pixels(5.0)).col_between(Pixels(10.0))

    }
}

impl<L, T> View for Grid<L, T> 
where L: 'static + Lens<Target = Vec<T>>
{
    fn body(&mut self, cx: &mut Context) {

        if let Some(builder) = self.builder.clone() {
            let mut found_store = None;
    
            'tree: for entity in cx.current.parent_iter(&cx.tree.clone()) {
                if let Some(model_list) = cx.data.model_data.get(entity) {
                    for (_, model) in model_list.iter() {
                        if let Some(store) = model.downcast_ref::<Store<L::Source>>() {
                            found_store = Some(store); 
                            break 'tree;
                        }
                    }
                }
            };
    
            if let Some(store) = found_store {
                
                let len = self.lens.view(&store.data).len();
    
                cx.style.borrow_mut().grid_rows.insert(cx.current, vec![Pixels(30.0); self.rows]);
                cx.style.borrow_mut().grid_cols.insert(cx.current, vec![Pixels(30.0); self.columns]);
    
                'loop_row: for row in 0..self.rows {
                    for col in 0..self.columns {
                        let index = row * self.columns + col;
                        let ptr = ItemPtr::new(self.lens.clone(), index, row, col);

                        let columns = self.columns;
                        let builder = builder.clone();
                        HStack::new(cx, move |cx|{
                            (builder)(cx, columns, ptr.clone());
                        }).row_index(row).col_index(col);
                        cx.count += 1;
                    }
                }
            }

        }


    }
}



#[derive(Lens)]
pub struct GridData {
    selected: (usize, usize),
}

impl Model for GridData {

}