using System.ComponentModel.Composition;
using System.Composition;
using HardHatC2Client.Utilities;
using static HardHatC2Client.Utilities.CommandItem;
using static HardHatC2Client.Utilities.CommandKey;


namespace Rivet_ClientPlugin
{
    [Export(typeof(ImplantCommandValidation_Base))]
    [ExportMetadata("Name", "Rivet")]
    [ExportMetadata("Description", "This is the rivet command validation")]
    public class RivetCommandValidation : ImplantCommandValidation_Base
    {
        public override List<CommandItem> CommandList { get; } = new List<CommandItem>()
        {
            new CommandItem()
            {
                Name = "whoami",
                Description = "a whoami command in rust :) currently just gets the active username",
                Usage = "whoami",
                NeedsAdmin = false,
                Opsec = OpsecStatus.NotSet,
                MitreTechnique = "",
                RequiresPreProc = false,
                RequiresPostProc = false,
                Keys = null,
            },
        };
    }
}
