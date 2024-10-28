import tkinter as tk
from tkinter import simpledialog, messagebox, filedialog
import csv
import subprocess  # To call the external program


class GridApp:
    def __init__(self, root, grid_size=10):
        self.root = root
        self.root.title("Grid Point App")

        # Set up grid parameters
        self.grid_size = grid_size
        self.cell_size = 50
        self.points = {}  # Dictionary to store points with their weights
        self.last_saved_file = None  # To store the last saved CSV file path

        # Create Canvas
        canvas_size = self.grid_size * 2 * self.cell_size
        self.canvas = tk.Canvas(
            self.root, width=canvas_size, height=canvas_size, bg="white"
        )
        self.canvas.pack()

        # Draw initial grid with center at (0, 0)
        self.draw_grid()

        # Bind click and motion events
        self.canvas.bind("<Button-1>", self.add_or_remove_point)
        self.canvas.bind("<Motion>", self.update_mouse_coords)

        # Add export button
        self.export_button = tk.Button(
            self.root, text="Export to CSV", command=self.export_to_csv
        )
        self.export_button.pack(pady=5)

        # Add button to call external program
        self.suggest_button = tk.Button(
            self.root, text="Suggest New Point", command=self.suggest_new_point
        )
        self.suggest_button.pack(pady=5)

        # Add label to show coordinates at bottom left
        self.coords_label = tk.Label(self.root, text="Mouse Coordinates: (0, 0)")
        self.coords_label.pack(anchor="w", padx=10, pady=5)

    def draw_grid(self):
        """Draws a grid centered at (0, 0) on the canvas."""
        center_offset = self.grid_size * self.cell_size

        for i in range(-self.grid_size, self.grid_size + 1):
            # Vertical lines
            self.canvas.create_line(
                center_offset + i * self.cell_size,
                0,
                center_offset + i * self.cell_size,
                2 * center_offset,
                fill="black",
            )
            # Horizontal lines
            self.canvas.create_line(
                0,
                center_offset + i * self.cell_size,
                2 * center_offset,
                center_offset + i * self.cell_size,
                fill="black",
            )

        # Draw axes
        self.canvas.create_line(
            center_offset, 0, center_offset, 2 * center_offset, fill="red", width=2
        )
        self.canvas.create_line(
            0, center_offset, 2 * center_offset, center_offset, fill="red", width=2
        )

    def add_or_remove_point(self, event):
        """Adds or removes a point with weight on the grid with (0, 0) at the center."""
        center_offset = self.grid_size * self.cell_size
        # Convert canvas coordinates to grid coordinates centered at (0, 0)
        x = (event.x - center_offset) // self.cell_size
        y = (center_offset - event.y) // self.cell_size

        if (
            x <= -self.grid_size
            or x >= self.grid_size
            or y <= -self.grid_size
            or y >= self.grid_size
        ):
            return

        # Check if point already exists
        if (x, y) in self.points:
            # Remove point from dictionary
            del self.points[(x, y)]
            # Clear the visual representation of the point
            self.redraw_grid()
            return

        # Prompt user for weight if adding a new point
        weight = simpledialog.askfloat(
            "Input", f"Enter weight for point ({x},{y})", minvalue=0.1
        )

        if weight is not None and weight > 0:
            self.redraw_grid()
            # Store the point and weight
            self.points[(x, y)] = weight
            # Draw a circle to represent the point
            self.canvas.create_oval(
                center_offset + (x - 0.25) * self.cell_size,
                center_offset - (y - 0.25) * self.cell_size,
                center_offset + (x + 0.25) * self.cell_size,
                center_offset - (y + 0.25) * self.cell_size,
                fill="blue",
            )
            # Display the weight
            self.canvas.create_text(
                center_offset + x * self.cell_size,
                center_offset - y * self.cell_size,
                text=str(weight),
                fill="white",
            )

    def redraw_grid(self):
        """Redraws the grid and all points."""
        self.canvas.delete("all")
        self.draw_grid()
        center_offset = self.grid_size * self.cell_size
        for (x, y), weight in self.points.items():
            self.canvas.create_oval(
                center_offset + (x - 0.25) * self.cell_size,
                center_offset - (y - 0.25) * self.cell_size,
                center_offset + (x + 0.25) * self.cell_size,
                center_offset - (y + 0.25) * self.cell_size,
                fill="blue",
            )
            self.canvas.create_text(
                center_offset + x * self.cell_size,
                center_offset - y * self.cell_size,
                text=str(weight),
                fill="white",
            )

    def update_mouse_coords(self, event):
        """Updates the coordinates label with the mouse's grid position."""
        center_offset = self.grid_size * self.cell_size
        x = (event.x - center_offset) // self.cell_size
        y = (center_offset - event.y) // self.cell_size
        self.coords_label.config(text=f"Mouse Coordinates: ({x}, {y})")

    def export_to_csv(self, file_path=None):
        """Exports the points and weights to a CSV file."""
        if not self.points:
            messagebox.showwarning("No Points", "There are no points to export.")
            return

        if file_path == None:
            file_path = filedialog.asksaveasfilename(
                defaultextension=".csv", filetypes=[("CSV files", "*.csv")]
            )
        if file_path:
            with open(file_path, mode="w", newline="") as file:
                writer = csv.writer(file)
                for (x, y), weight in self.points.items():
                    writer.writerow([x, y, weight])
            self.last_saved_file = file_path
            # messagebox.showinfo("Export Successful", f"Data exported to {file_path}")

    def suggest_new_point(self):
        """Calls an external program to get suggested coordinates for a new point."""
        self.export_to_csv("temp.csv")

        # Call the external program, passing the last saved file as an argument
        try:
            result = subprocess.check_output(
                ["../../rust/target/release/weighted_median", self.last_saved_file]
            )
            result = result.decode("utf-8").splitlines()[1]
            r = result.split()
            x = int(r[0])
            y = int(r[1])

            # Draw the red point at the suggested coordinates
            center_offset = self.grid_size * self.cell_size
            self.canvas.create_oval(
                center_offset + (x - 0.25) * self.cell_size,
                center_offset - (y - 0.25) * self.cell_size,
                center_offset + (x + 0.25) * self.cell_size,
                center_offset - (y + 0.25) * self.cell_size,
                fill="red",
            )

        except subprocess.CalledProcessError:
            messagebox.showerror(
                "Error", "Failed to get suggested point from external program."
            )
        except ValueError as e:
            print(e)
            messagebox.showerror("Error", "Invalid data from external program.")


# Run the application
if __name__ == "__main__":
    root = tk.Tk()
    app = GridApp(root, grid_size=5)  # Default grid size is 10x10
    root.mainloop()
