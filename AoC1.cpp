#include <iostream>
#include <fstream>
#include <vector>
#include <cctype>

std::vector<std::string> split_lines(char* buffer, int buffer_size) {
  std::vector<std::string> lines;
  char* start = buffer;
  char* end = buffer;

  while (end < buffer + buffer_size) {
    if (*end == '\n') {
      *end = '\0';
      lines.push_back(start);
      start = end + 1;
    }
    end++;
  }
  if (start < buffer + buffer_size) {
    lines.push_back(start);
  }
  return lines;
}

int main() {
  std::fstream file;
  file.open("puzzleinput.txt", std::ios::in);
  if (!file) {
    std::cout << "Error opening file." << std::endl;
    return 1; 
  }

  
  file.seekg(0, std::ios::end);
  std::streamsize size = file.tellg();
  char* buffer = new char[size + 1]; 
  file.seekg(0, std::ios::beg);
  file.read(buffer, size);
  buffer[size] = '\0';
  file.close();

  std::vector<std::string> lines = split_lines(buffer, size);

  int total_sum = 0;
  for (const std::string& line : lines) {
    
    if (line.size() < 2) {
      
      continue;
    }

    char first_digit = line[0];
    char last_digit = line[line.size() - 1];

    
    if (isdigit(first_digit) && isdigit(last_digit)) {
      int value = (first_digit - '0') * 10 + (last_digit - '0');
      total_sum += value;
    }
  }

  std::cout << "Sum of calibration values: " << total_sum << std::endl;

  
  delete[] buffer;

  return 0;
}
