using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Media;
using System.Windows.Media.Imaging;

namespace Heligman_Pollard_Visualizer
{
    public class MainViewModel : INotifyPropertyChanged
    {
        private const int width = 200;
        private const int height = 200;

        public WriteableBitmap MortalityImage { get; set; }

        public float InfantMortality { get; set; }
        public WriteableBitmap InfantMortalityImage { get; set; }

        public float FirstYearMortality { get; set; }
        public WriteableBitmap FirstYearMortalityImage { get; set; }

        public float InfantMortalityDropoff { get; set; }
        public WriteableBitmap InfantMortalityDropoffImage { get; set; }

        public float AccidentSeverity { get; set; }
        public WriteableBitmap AccidentSeverityImage { get; set; }

        public float AccidentSpread { get; set; }
        public WriteableBitmap AccidentSpreadImage { get; set; }

        public float AccidentMidpoint { get; set; }
        public WriteableBitmap AccidentMidpointImage { get; set; }

        public float AdultMortality { get; set; }
        public WriteableBitmap AdultMortalityImage { get; set; }

        public float AdultMortalityIncrease { get; set; }
        public WriteableBitmap AdultMortalityIncreaseImage { get; set; }

        public MainViewModel()
        {
            MortalityImage = new WriteableBitmap(
                850,
                375,
                96,
                96,
                PixelFormats.Rgb24,
                null
                );

            InfantMortality = (float)0.0004;

            InfantMortalityImage = new WriteableBitmap(
                width,
                height,
                96,
                96,
                PixelFormats.Rgb24,
                null
                );

            FirstYearMortality = (float)0.0192;

            FirstYearMortalityImage = new WriteableBitmap(
                width,
                height,
                96,
                96,
                PixelFormats.Rgb24,
                null
                );

            InfantMortalityDropoff = (float)0.1048;

            InfantMortalityDropoffImage = new WriteableBitmap(
                width,
                height,
                96,
                96,
                PixelFormats.Rgb24,
                null
                );

            AccidentSeverity = (float)0.001;

            AccidentSeverityImage = new WriteableBitmap(
                width,
                height,
                96,
                96,
                PixelFormats.Rgb24,
                null
                );

            AccidentSpread = (float)9.0;

            AccidentSpreadImage = new WriteableBitmap(
                width,
                height,
                96,
                96,
                PixelFormats.Rgb24,
                null
                );

            AccidentMidpoint = (float)21.0;

            AccidentMidpointImage = new WriteableBitmap(
                width,
                height,
                96,
                96,
                PixelFormats.Rgb24,
                null
                );

            AdultMortality = (float)0.0001;

            AdultMortalityImage = new WriteableBitmap(
                width,
                height,
                96,
                96,
                PixelFormats.Rgb24,
                null
                );

            AdultMortalityIncrease = (float)1.1;

            AdultMortalityIncreaseImage = new WriteableBitmap(
                width,
                height,
                96,
                96,
                PixelFormats.Rgb24,
                null
                );

        }

        public event PropertyChangedEventHandler? PropertyChanged;
    }
}
