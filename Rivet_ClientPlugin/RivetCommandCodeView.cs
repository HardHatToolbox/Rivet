using System;
using System.Collections.Generic;
using System.ComponentModel.Composition;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using HardHatC2Client.Plugin_BaseClasses;

namespace Rivet_ClientPlugin
{
    [Export(typeof(CommandCodeView_Base))]
    [ExportMetadata("Name", "Rivet")]
    [ExportMetadata("Description", "This is the rivet command code view")]
    [ExportMetadata("CodeLang","rs")]
    [ExportMetadata("ImplantSrc", "..\\Rivet\\src\\tasking\\commands")]
    internal class RivetCommandCodeView : CommandCodeView_Base
    {
    }
}
