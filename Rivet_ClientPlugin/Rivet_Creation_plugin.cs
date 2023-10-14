using HardHatC2Client.Plugin_BaseClasses;
using System.ComponentModel.Composition;
using System.Composition;


namespace Rivet_ClientPlugin
{
    [Export(typeof(ImplantCreation_Base))]
    [ExportMetadata("Name", "Rivet")]
    [ExportMetadata("Description", "This is the rivet implant creation")]
    public class Rivet_Creation_plugin : ImplantCreation_Base
    {
        public override Type GetComponentType()
        {
            return typeof(Rivet_creation_comp);
        }
    }
}
