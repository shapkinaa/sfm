pub struct Store<TState, TAction>
where
    TState: Default + Clone,
    TAction: Clone,
{
    state: TState,
    root_reducer: RootReducer<TState, TAction>,
    listeners: Vec<Listener<TState>>,
    middlewares: Vec<Middleware<TState, TAction>>,
}

impl<TState, TAction> Store<TState, TAction>
where
    TState: Default + Clone,
    TAction: Clone,
{
    pub fn new(root_reducer: RootReducer<TState, TAction>) -> Self {
        Store::with_state(root_reducer, TState::default())
    }

    pub fn with_state(root_reducer: RootReducer<TState, TAction>, state: TState) -> Self {
        Store {
            state,
            root_reducer,
            listeners: Vec::new(),
            middlewares: Vec::new(),
        }
    }

    pub fn get_state(&self) -> TState {
        self.state.clone()
    }

    pub fn dispatch(&mut self, action: TAction) {
        if self.middlewares.is_empty() == false {
            self.dispatch_middlewares(action);
        } else {
            self.state = self.dispatch_reducer(action);
        }
    }

    pub fn register_listener(&mut self, listener: Listener<TState>) {
        self.listeners.push(listener);
    }

    pub fn register_middleware(&mut self, middleware: Middleware<TState, TAction>) {
        self.middlewares.push(middleware);
    }

    fn dispatch_reducer(&self, action: TAction) -> TState {
        (self.root_reducer)(self.state.clone(), action)
    }

    fn dispatch_middlewares(&mut self, action: TAction) {
        while let Some(middleware) = self.middlewares.iter().next() {
            if let Some(middleware_action) = middleware(self, action.clone()) {
                self.state = self.dispatch_reducer(middleware_action);
            }
        }
    }
}

type RootReducer<TState, TAction> = fn(TState, TAction) -> TState;
type Listener<TState> = fn(&TState);
type Middleware<TState, TAction> = fn(&mut Store<TState, TAction>, TAction) -> Option<TAction>;
