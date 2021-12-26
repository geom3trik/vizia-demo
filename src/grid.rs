use std::{collections::HashSet, marker::PhantomData, rc::Rc, any::TypeId};

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
where 
    L: Lens<Target = Vec<T>>,
    T: Data,
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
        let ancestors = parent.parent_iter(&cx.tree).collect::<HashSet<_>>();

        for entity in id.parent_iter(&cx.tree) {
            if let Some(model_data_store) = cx.data.model_data.get_mut(entity) {
                if let Some(model_data) = model_data_store.data.get(&TypeId::of::<L::Source>()) {
                    if let Some(lens_wrap) = model_data_store.lenses.get_mut(&TypeId::of::<L>()) {
                        let observers = lens_wrap.observers();

                        if ancestors.intersection(observers).next().is_none() {
                            lens_wrap.add_observer(id);
                        }
                    } else {
                        let mut observers = HashSet::new();
                        observers.insert(id);

                        let model = model_data.downcast_ref::<Store<L::Source>>().unwrap();

                        let old = lens.view(&model.data);

                        model_data_store.lenses.insert(
                            TypeId::of::<L>(),
                            Box::new(StateStore { entity: id, lens, old: old.clone(), observers }),
                        );
                    }

                    break;
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
                    for (_, model) in model_list.data.iter() {
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
                        //cx.count += 1;
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