using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;
using System.Runtime.InteropServices;

namespace Heligman_Pollard_Visualizer
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        MainViewModel viewModel = new MainViewModel();
        public MainWindow()
        {
            InitializeComponent();
            DataContext = viewModel;

            RenderImages();
        }

        [DllImport("./heligman_pollard_image_gen.dll")]
        private unsafe static extern Graphs plot_8d_graphs(
            float infant_mortality,
            float first_year_mortality,
            float infant_mortality_dropoff,
            float accident_severity,
            float accident_spread,
            float accident_midpoint,
            float adult_mortality,
            float adult_mortality_increase
            );

        [DllImport("./heligman_pollard_image_gen.dll")]
        private unsafe static extern void free_buffer(Buffer buffer);

        public void RenderImages() {
            Graphs graphs = plot_8d_graphs(
                viewModel.InfantMortality,
                viewModel.FirstYearMortality,
                viewModel.InfantMortalityDropoff,
                viewModel.AccidentSeverity,
                viewModel.AccidentSpread,
                viewModel.AccidentMidpoint,
                viewModel.AdultMortality,
                viewModel.AdultMortalityIncrease
                );

            LoadBuffer(viewModel.MortalityImage, graphs.mortality);
            LoadBuffer(viewModel.InfantMortalityImage, graphs.infant_mortality);
            LoadBuffer(viewModel.FirstYearMortalityImage, graphs.first_year_mortality);
            LoadBuffer(viewModel.InfantMortalityDropoffImage, graphs.infant_mortality_dropoff);
            LoadBuffer(viewModel.AccidentSeverityImage, graphs.accident_severity);
            LoadBuffer(viewModel.AccidentSpreadImage, graphs.accident_spread);
            LoadBuffer(viewModel.AccidentMidpointImage, graphs.accident_midpoint);
            LoadBuffer(viewModel.AdultMortalityImage, graphs.adult_mortality);
            LoadBuffer(viewModel.AdultMortalityIncreaseImage, graphs.adult_mortality_increase);

        }

        public static void LoadBuffer(WriteableBitmap bitmap, Buffer buf) {
            unsafe {
                bitmap.Lock();
                byte* bitmapPointer = (byte*)bitmap.BackBuffer.ToPointer();
                nuint height = (nuint)bitmap.PixelHeight;
                nuint width = (nuint)bitmap.PixelWidth;
                nuint rowSize = (nuint)bitmap.BackBufferStride;
                for (nuint y = 0; y < height; ++y) {
                    for (nuint x = 0;  x < width*3; ++x)
                    {
                        bitmapPointer[x + y * rowSize] = (byte)buf.data[x + y * width*3];
                    }
                }
                bitmap.AddDirtyRect(new Int32Rect(0, 0, (int)width, (int)height));
                bitmap.Unlock();

                free_buffer(buf);
            }
        }

        private void infantMortalitySlider_ValueChanged(object sender, RoutedPropertyChangedEventArgs<double> e)
        {
            viewModel.InfantMortality = (float)e.NewValue;
            RenderImages();
        }

        private void firstYearMortalitySlider_ValueChanged(object sender, RoutedPropertyChangedEventArgs<double> e)
        {
            viewModel.FirstYearMortality = (float)e.NewValue;
            RenderImages();
        }

        private void infantMortalityDropoffSlider_ValueChanged(object sender, RoutedPropertyChangedEventArgs<double> e)
        {
            viewModel.InfantMortalityDropoff = (float)e.NewValue;
            RenderImages();
        }

        private void accidentSeveritySlider_ValueChanged(object sender, RoutedPropertyChangedEventArgs<double> e)
        {
            viewModel.AccidentSeverity = (float)e.NewValue;
            RenderImages();
        }

        private void accidentSpreadSlider_ValueChanged(object sender, RoutedPropertyChangedEventArgs<double> e)
        {
            viewModel.AccidentSpread = (float)e.NewValue;
            RenderImages();
        }

        private void accidentMidpointSlider_ValueChanged(object sender, RoutedPropertyChangedEventArgs<double> e)
        {
            viewModel.AccidentMidpoint = (float)e.NewValue;
            RenderImages();
        }

        private void adultMortalitySlider_ValueChanged(object sender, RoutedPropertyChangedEventArgs<double> e)
        {
            viewModel.AdultMortality = (float)e.NewValue;
            RenderImages();
        }

        private void adultMortalityIncreaseSlider_ValueChanged(object sender, RoutedPropertyChangedEventArgs<double> e)
        {
            viewModel.AdultMortalityIncrease = (float)e.NewValue;
            RenderImages();
        }
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct Buffer {
        public byte* data;
        public nuint len;
        public nuint capacity;
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct Graphs {
        public Buffer mortality;
        public Buffer infant_mortality;
        public Buffer first_year_mortality;
        public Buffer infant_mortality_dropoff;
        public Buffer accident_severity;
        public Buffer accident_spread;
        public Buffer accident_midpoint;
        public Buffer adult_mortality;
        public Buffer adult_mortality_increase;
    }
}
