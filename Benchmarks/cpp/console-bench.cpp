
#include <algorithm>
#include <chrono>
#include <iomanip>
#include <iostream>
#include <memory>
#include <vector>

#include <libgen.h>

#include "faust/gui/UI.h"
#include "faust/dsp/dsp.h"  // or "faust/dsp/llvm-dsp.h" ?
#include "faust/misc.h"

<<includeIntrinsic>>

<<includeclass>>

//-------------------------------------------------------------------------
// 									MAIN
//-------------------------------------------------------------------------

int main(int argc, char *argv[])
{
    int buffer_size = 1024;
    int sample_rate = 44100;
    int min_samples = sample_rate * 60 * 3;

    std::vector<double> throughputs;

    for (int i = 0; i < 10; ++i) {
        auto dsp = std::make_unique<mydsp>();
        dsp->init(sample_rate);

        int num_inputs = dsp->getNumInputs();
        int num_outputs = dsp->getNumOutputs();

        // Prepare buffers
        float** in_buffer = new float* [num_inputs];
        for (int i = 0; i < num_inputs; ++i) {
            in_buffer[i] = new float[buffer_size];
        }
        float** out_buffer = new float* [num_outputs];
        for (int i = 0; i < num_outputs; ++i) {
            out_buffer[i] = new float[buffer_size];
        }

        // Set input buffers to fixed impulse responses
        for (int c = 0; c < num_inputs; ++c) {
            for (int j = 0; j < buffer_size; ++j) {
                in_buffer[c][j] = (j == 0 ? 1.0 : 0.0);
            }
        }

        // Compute
        float sample_sum = 0.0;
        int num_samples_written = 0;

        std::chrono::steady_clock::time_point t1 = std::chrono::steady_clock::now();
        while (num_samples_written < min_samples) {
            dsp->compute(buffer_size, in_buffer, out_buffer);

            // Lightweight result access to prevent overoptimizations
            for (int c = 0; c < num_outputs; ++c) {
                sample_sum += out_buffer[c][0];
            }
            num_samples_written += buffer_size;
        }
        std::chrono::steady_clock::time_point t2 = std::chrono::steady_clock::now();

        auto elapsed_ns = std::chrono::duration_cast<std::chrono::nanoseconds>(t2 - t1).count();
        auto elapsed = (double) elapsed_ns / 1e9;
        auto audio_length = (double) num_samples_written / (double) sample_rate;
        auto load = 100.0 * elapsed / audio_length;
        auto throughput = double(num_samples_written * 4 * num_outputs) / double(elapsed);

        throughputs.emplace_back(throughput);

        std::cout <<
            "Rendered audio of length " << audio_length <<
            " sec in " << elapsed <<
            " sec [load: " << load << " %]    " <<
            throughput / 1024 / 1024 << " MB/sec" <<
            "    sample checksum: " << sample_sum << "\n";

        // Cleanup buffers
        for (int i = 0; i < num_inputs; ++i) {
            delete [] in_buffer[i];
        }
        delete [] in_buffer;
        for (int i = 0; i < num_outputs; ++i) {
            delete [] out_buffer[i];
        }
        delete [] out_buffer;
    }

    // print throughput stats
    double min = *std::min_element(std::begin(throughputs), std::end(throughputs));
    double max = *std::max_element(std::begin(throughputs), std::end(throughputs));

    size_t mid = throughputs.size() / 2;
    std::sort(throughputs.begin(), throughputs.end());
    double median = (
        throughputs.size() % 2 == 0 ?
        (throughputs[mid] + throughputs[mid + 1]) / 2 :
        throughputs[mid]
    );
    std::cout << std::fixed << std::setprecision(3);
    std::cout << "\n";
    std::cout << "Throughput min:    " << min / 1024 / 1024 << " MB/sec" << std::endl;
    std::cout << "Throughput median: " << median / 1024 / 1024 << " MB/sec" << std::endl;
    std::cout << "Throughput max:    " << max / 1024 / 1024 << " MB/sec" << std::endl;

    return 0;
}
