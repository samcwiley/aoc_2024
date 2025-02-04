#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>
#include <algorithm>
#include <map>

enum class BlockType
{
    Empty,
    File
};

struct Block
{
    BlockType type;
    uint16_t id;

    Block() : type(BlockType::Empty), id(0) {}
    Block(uint16_t fileId) : type(BlockType::File), id(fileId) {}

    bool isEmpty() const { return type == BlockType::Empty; }

    friend std::ostream &operator<<(std::ostream &os, const Block &block)
    {
        os << (block.isEmpty() ? "." : std::to_string(block.id));
        return os;
    }
};

struct Disk
{
    std::vector<Block> data;

    void compress()
    {
        size_t i = 0, j = data.size() - 1;
        while (i < j)
        {
            while (i < data.size() && !data[i].isEmpty())
                ++i;
            while (j > 0 && data[j].isEmpty())
                --j;
            if (i < j)
                std::swap(data[i], data[j]);
        }
    }

    uint64_t checksum() const
    {
        uint64_t result = 0;
        for (size_t i = 0; i < data.size(); ++i)
        {
            if (!data[i].isEmpty())
            {
                result += i * data[i].id;
            }
        }
        return result;
    }

    void move_files_left()
    {
        // Step 1: Identify file segments and store by decreasing file ID
        std::map<uint16_t, std::pair<size_t, size_t>, std::greater<uint16_t>> files;
        size_t start = 0;

        while (start < data.size())
        {
            if (!data[start].isEmpty())
            {
                uint16_t id = data[start].id;
                size_t end = start;

                while (end < data.size() && data[end].id == id)
                {
                    ++end;
                }

                files[id] = {start, end - start};
                start = end;
            }
            else
            {
                ++start;
            }
        }

        // Step 2: Move files to the leftmost available span
        for (const auto &[id, info] : files)
        {
            size_t file_start = info.first;
            size_t file_length = info.second;

            // Find the leftmost free space large enough to fit the file
            size_t best_pos = data.size(); // Initially, no valid position
            size_t i = 0;

            while (i < file_start)
            {
                if (data[i].isEmpty())
                {
                    // Check if this span is large enough
                    size_t j = i;
                    while (j < data.size() && data[j].isEmpty())
                    {
                        ++j;
                        if (j - i >= file_length)
                        {
                            best_pos = i;
                            break;
                        }
                    }
                }
                ++i;
            }

            // Move the file if a valid position was found
            if (best_pos != data.size())
            {
                // Clear old file location
                std::fill(data.begin() + file_start, data.begin() + file_start + file_length, Block());

                // Place the file in the new position
                std::fill(data.begin() + best_pos, data.begin() + best_pos + file_length, Block(id));
            }
        }
    }
};

using Input = std::vector<uint8_t>;

Input parse_input(const std::string &raw)
{
    Input input;
    std::stringstream ss(raw);
    char c;
    while (ss >> c)
    {
        if (isdigit(c))
            input.push_back(c - '0');
    }
    return input;
}

Disk expand_input(const Input &input)
{
    Disk disk;
    if (input.empty())
        return disk;

    uint16_t id = 1;
    for (int i = 0; i < input[0]; ++i)
        disk.data.emplace_back(0);

    for (size_t i = 1; i + 1 < input.size(); i += 2)
    {
        uint8_t space_length = input[i], file_length = input[i + 1];
        for (int j = 0; j < space_length; ++j)
            disk.data.emplace_back();
        for (int j = 0; j < file_length; ++j)
            disk.data.emplace_back(id);
        ++id;
    }
    return disk;
}

std::string part1(const Input &input)
{
    Disk disk = expand_input(input);
    disk.compress();
    return std::to_string(disk.checksum());
}

std::string part2(const Input &input)
{
    Disk disk = expand_input(input);
    disk.move_files_left();
    return std::to_string(disk.checksum());
}

// 🔹 **Main Function to Read File & Execute**
int main()
{
    std::ifstream file("../../src/.inputs/input09.txt");
    if (!file)
    {
        std::cerr << "Error: Could not open input file!\n";
        return 1;
    }

    std::stringstream buffer;
    buffer << file.rdbuf();
    std::string input_data = buffer.str();

    Input data = parse_input(input_data);
    std::string result1 = part1(data);
    std::string result2 = part2(data);

    std::cout << "Part 1 result: " << result1 << "\n";
    std::cout << "Part 2 result: " << result2 << "\n";
    return 0;
}
