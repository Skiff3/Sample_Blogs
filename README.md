# MVC Blog Application

## Overview
This is a **scalable and efficient** blog management system built using the **Rust Axum framework**, following the **Model-View-Controller (MVC) architecture**. The application enables users to create, edit, and manage blog posts with an intuitive interface using **HTML and Handlebars** templates.

## Features
- **User & Admin Interface**: Separate interfaces for users and administrators.
- **Blog Management**: Create, update, delete, and list blog posts.
- **Database Integration**: Utilizes PostgreSQL for efficient data storage.
- **Templating Engine**: Uses Handlebars for dynamic HTML rendering.
- **RESTful API**: Provides endpoints for blog management.
- **Authentication & Authorization**: Secure access for different user roles.
- **Scalable Architecture**: Built with Rust and Axum for high performance.

## Tech Stack
- **Backend**: Rust (Axum framework, Tokio, Serde, sqlx)
- **Frontend**: HTML, Handlebars
- **Database**: PostgreSQL
- **Authentication**: JWT-based authentication
- **Deployment**: Docker & AWS

## Installation
1. **Clone the repository**
   ```sh
   git clone https://github.com/your-repo/mvc-blog.git
   cd mvc-blog
   ```

2. **Set up the database** (PostgreSQL required)
   ```sh
   export DATABASE_URL=postgres://user:password@localhost/blog_db
   ```

3. **Run migrations**
   ```sh
   cargo install sqlx-cli
   sqlx migrate run
   ```

4. **Build and Run the application**
   ```sh
   cargo run
   ```

## Usage
- Access the blog at `http://localhost:8000`
- Create, edit, and manage blog posts via the UI.
- Use the provided RESTful API for integration with other services.

## API Endpoints
- `GET /blogs` - Fetch all blogs
- `POST /blogs` - Create a new blog post
- `PUT /blogs/:id` - Update a blog post
- `DELETE /blogs/:id` - Delete a blog post

## Contributing
1. Fork the repository
2. Create a feature branch (`git checkout -b feature-name`)
3. Commit your changes (`git commit -m "Add new feature"`)
4. Push to the branch (`git push origin feature-name`)
5. Create a pull request

## License
This project is licensed under the **MIT License**. Feel free to use and modify it.
