use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::sync::{Mutex, Arc};
use serde::{Serialize, Deserialize};

const GRID_SIZE: usize = 16;

#[derive(Serialize, Deserialize, Clone)]
struct Grid {
    cells: Vec<u8>, // brightness values (0-255)
}

impl Grid {
    fn new() -> Self {
        Grid { cells: vec![255; GRID_SIZE * GRID_SIZE] }
    }
}

type SharedGrid = Arc<Mutex<Grid>>;

// Handler for homepage (unchanged)
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(r#"
            <h1 style='text-align: center;'>Welcome to Big Letters!</h1>
            <p style='text-align: center;'>
                Check out some big letters: 
                <a href='/a'>A</a> | 
                <a href='/b'>B</a> | 
                <a href='/c'>C</a>
            </p>
        "#)
}

// Handler for page A with interactive grid
async fn page_a() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(r#"
            <h1 style='font-size: 200px; text-align: center;'>A</h1>
            <div style='text-align: center;'>
                <form action='/' method='get'>
                    <button type='submit' style='font-size: 24px;'>Go Home</button>
                </form>
            </div>
            <div id='grid' style='
                display: grid;
                grid-template-columns: repeat(16, 20px);
                grid-template-rows: repeat(16, 20px);
                gap: 2px;
                justify-content: center;
                margin-top: 40px;
            '>
            </div>
            <script>
                async function fetchGrid() {
                    const res = await fetch('/api/grid');
                    return await res.json();
                }
                async function updateCell(idx) {
                    await fetch('/api/cell', {
                        method: 'POST',
                        headers: {'Content-Type': 'application/json'},
                        body: JSON.stringify({ idx })
                    });
                    renderGrid();
                }
                async function renderGrid() {
                    const gridDiv = document.getElementById('grid');
                    const grid = await fetchGrid();
                    gridDiv.innerHTML = '';
                    for (let i = 0; i < grid.cells.length; i++) {
                        const cell = document.createElement('div');
                        const brightness = grid.cells[i];
                        cell.style.width = '20px';
                        cell.style.height = '20px';
                        cell.style.background = `rgb(${brightness},${brightness},${brightness})`;
                        cell.style.border = '1px solid #ccc';
                        cell.onclick = () => updateCell(i);
                        gridDiv.appendChild(cell);
                    }
                }
                renderGrid();
            </script>
        "#)
}

// API: Get grid state
async fn get_grid(grid: web::Data<SharedGrid>) -> impl Responder {
    let grid = grid.lock().unwrap();
    HttpResponse::Ok().json(grid.clone())
}

// API: Update cell brightness
#[derive(Deserialize)]
struct CellUpdate {
    idx: usize,
}

async fn update_cell(
    grid: web::Data<SharedGrid>,
    info: web::Json<CellUpdate>,
) -> impl Responder {
    let mut grid = grid.lock().unwrap();
    if info.idx < grid.cells.len() {
        let b = grid.cells[info.idx].saturating_sub(32);
        grid.cells[info.idx] = b;
    }
    HttpResponse::Ok().finish()
}

// Handlers for B and C (unchanged)
async fn page_b() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(r#"
            <h1 style='font-size: 200px; text-align: center;'>B</h1>
            <div style='text-align: center;'>
                <form action='/' method='get'>
                    <button type='submit' style='font-size: 24px;'>Go Home</button>
                </form>
            </div>
        "#)
}

async fn page_c() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(r#"
            <h1 style='font-size: 200px; text-align: center;'>C</h1>
            <div style='text-align: center;'>
                <form action='/' method='get'>
                    <button type='submit' style='font-size: 24px;'>Go Home</button>
                </form>
            </div>
        "#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let grid = Arc::new(Mutex::new(Grid::new()));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(grid.clone()))
            .route("/", web::get().to(index))
            .route("/a", web::get().to(page_a))
            .route("/b", web::get().to(page_b))
            .route("/c", web::get().to(page_c))
            .route("/api/grid", web::get().to(get_grid))
            .route("/api/cell", web::post().to(update_cell))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}