use wee_alloc::WeeAlloc;

mod board;
mod game;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;
