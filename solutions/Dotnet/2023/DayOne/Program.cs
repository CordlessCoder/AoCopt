using System.Diagnostics;

#if DEBUG
var file = File.ReadAllLines("../../../inputs.txt");
var input = string.Join('\n', file).Trim();
#else
using var sr = new StreamReader(Console.OpenStandardInput(), Console.InputEncoding);
var input = sr.ReadToEnd().Trim();
#endif

var watch = new Stopwatch();
watch.Start();

var sum = input.Split('\n')
        .Select(line =>
        {
                var first = line.First(char.IsDigit) - '0';
                var last = line.Last(char.IsDigit) - '0';
                return first * 10 + last;
        })
        .Sum();

watch.Stop();
var frequency = Stopwatch.Frequency;
var nanosecPerTick = (1000L*1000L*1000L) / frequency;

Console.WriteLine(sum);
Console.WriteLine($"Took {watch.ElapsedTicks * nanosecPerTick} nanoseconds");
