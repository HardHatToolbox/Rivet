using System.ComponentModel.Composition;
using HardHatCore.HardHatC2Client.Plugin_BaseClasses;

namespace Rivet_ClientPlugin
{
    //This powers the command view UI component in hardhat, if you want users to be able to view command source code and make modifications straight in the UI, you need to implement this
    [Export(typeof(CommandCodeView_Base))]
    [ExportMetadata("Name", "Rivet")]
    [ExportMetadata("Description", "This is the rivet command code view")]
    [ExportMetadata("CodeLang","rs")]
    //relative path to the command source code from the HardHat base directory
    [ExportMetadata("ImplantSrc", "..\\Assets\\RivetAsset\\src\\tasking\\commands")]
    internal class RivetCommandCodeView : CommandCodeView_Base
    {
    }
}
