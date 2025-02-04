#include <iostream>
#include <vector>
#include <fstream>
#include <sstream>
#include <optional>
#include <unordered_set>

enum class Direction
{
    Up,
    Down,
    Left,
    Right
};

struct Hit
{
    std::unordered_set<Direction> hit_directions;

    bool hit(Direction dir)
    {
        if (hit_directions.count(dir))
        {
            return true;
        }
        hit_directions.insert(dir);
        return false;
    }
};

enum class GridObjectType
{
    Empty,
    Obstacle,
    Guy,
    Visited
};

struct GridObject
{
    GridObjectType type;
    std::optional<Direction> direction; // Only for 'Guy'
    std::optional<Hit> hit_data;        // Only for 'Obstacle'

    GridObject(GridObjectType type) : type(type) {}
    GridObject(Direction dir) : type(GridObjectType::Guy), direction(dir) {}
    GridObject(Hit hit) : type(GridObjectType::Obstacle), hit_data(hit) {}

    // Overloading << for easy printing
    friend std::ostream &operator<<(std::ostream &os, const GridObject &obj)
    {
        switch (obj.type)
        {
        case GridObjectType::Guy:
            switch (obj.direction.value_or(Direction::Up))
            {
            case Direction::Up:
                os << '^';
                break;
            case Direction::Down:
                os << 'v';
                break;
            case Direction::Left:
                os << '<';
                break;
            case Direction::Right:
                os << '>';
                break;
            }
            break;
        case GridObjectType::Obstacle:
            os << '#';
            break;
        case GridObjectType::Empty:
            os << '.';
            break;
        case GridObjectType::Visited:
            os << '*';
            break;
        }
        return os;
    }
};

class Grid
{
public:
    std::vector<std::vector<GridObject>> data;
    size_t width, height;

    Grid(size_t w, size_t h) : width(w), height(h), data(h, std::vector<GridObject>(w, GridObject(GridObjectType::Empty))) {}

    static std::optional<Grid> parse_grid(const std::string &input)
    {
        std::vector<std::vector<GridObject>> parsed_data;
        std::istringstream stream(input);
        std::string line;

        while (std::getline(stream, line))
        {
            std::vector<GridObject> row;
            for (char ch : line)
            {
                switch (ch)
                {
                case '.':
                    row.emplace_back(GridObjectType::Empty);
                    break;
                case '#':
                    row.emplace_back(Hit());
                    break;
                case '^':
                    row.emplace_back(Direction::Up);
                    break;
                default:
                    row.emplace_back(GridObjectType::Empty);
                    break;
                }
            }
            parsed_data.push_back(row);
        }

        if (parsed_data.empty() || parsed_data[0].empty())
            return std::nullopt;

        size_t width = parsed_data[0].size();
        for (const auto &row : parsed_data)
            if (row.size() != width)
                return std::nullopt; // Grid must be uniform

        Grid grid(width, parsed_data.size());
        grid.data = std::move(parsed_data);
        return grid;
    }

    std::optional<std::pair<size_t, size_t>> find_guy() const
    {
        for (size_t y = 0; y < height; ++y)
            for (size_t x = 0; x < width; ++x)
                if (data[y][x].type == GridObjectType::Guy)
                    return std::make_pair(x, y);
        return std::nullopt;
    }

    std::optional<int> run_grid(std::pair<size_t, size_t> coords)
    {
        size_t guy_x = coords.first, guy_y = coords.second;
        int spaces = 0;

        while (true)
        {
            auto &current = data[guy_y][guy_x];
            Direction dir = current.direction.value_or(Direction::Up);

            int dx = 0, dy = 0;
            switch (dir)
            {
            case Direction::Up:
                dy = -1;
                break;
            case Direction::Down:
                dy = 1;
                break;
            case Direction::Left:
                dx = -1;
                break;
            case Direction::Right:
                dx = 1;
                break;
            }

            int new_x = guy_x + dx, new_y = guy_y + dy;
            if (new_x < 0 || new_y < 0 || static_cast<size_t>(new_x) >= width || static_cast<size_t>(new_y) >= height)
                break; // Exiting the grid

            auto &next = data[new_y][new_x];

            if (next.type == GridObjectType::Visited)
            {
                current.type = GridObjectType::Visited;
                next = GridObject(dir);
                guy_x = new_x;
                guy_y = new_y;
            }
            else if (next.type == GridObjectType::Empty)
            {
                current.type = GridObjectType::Visited;
                next = GridObject(dir);
                guy_x = new_x;
                guy_y = new_y;
                spaces++;
            }
            else if (next.type == GridObjectType::Obstacle)
            {
                // Check for loop: if this obstacle has been hit from this direction before
                if (next.hit_data->hit(dir))
                {
                    return std::nullopt; // Loop detected!
                }

                // Turn right
                Direction new_dir;
                switch (dir)
                {
                case Direction::Up:
                    new_dir = Direction::Right;
                    break;
                case Direction::Right:
                    new_dir = Direction::Down;
                    break;
                case Direction::Down:
                    new_dir = Direction::Left;
                    break;
                case Direction::Left:
                    new_dir = Direction::Up;
                    break;
                }
                current = GridObject(new_dir);
            }
        }
        return spaces;
    }

    int part_2(std::pair<size_t, size_t> coords)
    {
        Grid grid = *this; // Clone the grid
        size_t guy_x = coords.first, guy_y = coords.second;
        int obstacles = 0;

        while (true)
        {
            auto &current = grid.data[guy_y][guy_x];
            Direction dir = current.direction.value_or(Direction::Up);

            int dx = 0, dy = 0;
            switch (dir)
            {
            case Direction::Up:
                dy = -1;
                break;
            case Direction::Down:
                dy = 1;
                break;
            case Direction::Left:
                dx = -1;
                break;
            case Direction::Right:
                dx = 1;
                break;
            }

            int new_x = guy_x + dx, new_y = guy_y + dy;
            if (new_x < 0 || new_y < 0 || static_cast<size_t>(new_x) >= width || static_cast<size_t>(new_y) >= height)
                break; // Exiting the grid

            auto &next = grid.data[new_y][new_x];

            if (next.type == GridObjectType::Visited)
            {
                grid.data[guy_y][guy_x] = GridObjectType::Visited;
                guy_x = new_x;
                guy_y = new_y;
            }
            else if (next.type == GridObjectType::Empty)
            {
                Grid new_grid = *this;                           // Clone grid to test obstacle placement
                new_grid.data[new_y][new_x] = GridObject(Hit()); // Place obstacle

                // Check if placing the obstacle creates a loop
                if (!new_grid.run_grid(coords))
                {
                    std::cout << "found a loop!\n";
                    obstacles++;
                }
                else
                {
                    std::cout << "no loop found; moving on\n";
                }

                // Continue movement
                grid.data[guy_y][guy_x] = GridObjectType::Visited;
                grid.data[new_y][new_x] = GridObject(dir);
                guy_x = new_x;
                guy_y = new_y;
            }
            else if (next.type == GridObjectType::Obstacle)
            {
                // If we hit an obstacle, we should now also check for loops
                if (next.hit_data->hit(dir))
                {
                    return obstacles; // End function if we're in a loop
                }

                // Turn right
                Direction new_dir;
                switch (dir)
                {
                case Direction::Up:
                    new_dir = Direction::Right;
                    break;
                case Direction::Right:
                    new_dir = Direction::Down;
                    break;
                case Direction::Down:
                    new_dir = Direction::Left;
                    break;
                case Direction::Left:
                    new_dir = Direction::Up;
                    break;
                }
                grid.data[guy_x][guy_y] = GridObject(new_dir);
            }
        }
        return obstacles;
    }

    void print_grid() const
    {
        for (const auto &row : data)
        {
            for (const auto &obj : row)
            {
                std::cout << obj;
            }
            std::cout << '\n';
        }
        std::cout << "\n";
    }
};

std::string read_input(const std::string &filename)
{
    std::ifstream file(filename);
    if (!file)
    {
        std::cerr << "Failed to open input file.\n";
        return "";
    }
    return std::string((std::istreambuf_iterator<char>(file)), std::istreambuf_iterator<char>());
}

int main()
{
    // std::string input = read_input("../../src/.inputs/input06.txt");
    std::string input = R"(....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...)";
    /*std::string input = R"(....#.....
....+...+#
..........
..#.......
.......#..
..........
.#.#^...+.
........#.
#.........
......#...)";*/
    auto grid_opt = Grid::parse_grid(input);

    if (!grid_opt)
    {
        std::cerr << "Failed to parse grid.\n";
        return 1;
    }

    Grid grid = *grid_opt;
    auto guy_coords = grid.find_guy();

    if (!guy_coords)
    {
        std::cerr << "Could not find the Guy on the grid.\n";
        return 1;
    }

    // grid.print_grid();
    auto part1_result = grid.run_grid(*guy_coords);
    if (part1_result)
    {
        std::cout << "Part 1 result: " << (*part1_result + 1) << '\n';
    }
    else
    {
        std::cout << "Guy got stuck in a loop!\n";
    }

    auto part2_result = grid.part_2(*guy_coords);
    std::cout << "Part 2 result: " << part2_result << '\n';

    return 0;
}
