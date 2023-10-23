using System.ComponentModel.Composition;
using HardHatCore.HardHatC2Client.Utilities;
using static HardHatCore.HardHatC2Client.Utilities.CommandItem;
using static HardHatCore.HardHatC2Client.Utilities.CommandKey;


namespace Rivet_ClientPlugin
{
    [Export(typeof(IImplantCommandValidation))]
    [ExportMetadata("Name", "Rivet")]
    [ExportMetadata("Description", "This is the rivet command validation")]
    public class RivetCommandValidation : IImplantCommandValidation
    {
        private readonly ImplantCommandValidation_Base implantCommandValidation_Base = new();

        public bool ValidateCommand(string input, out Dictionary<string, string> args, out string error)
        {
            return implantCommandValidation_Base.ValidateCommand(input, out args, out error);
        }

        public List<string> GetPostExCommands()
        {
            return implantCommandValidation_Base.GetPostExCommands();
        }

        public List<string> GetOptionalModules()
        {
            return implantCommandValidation_Base.GetOptionalModules();
        }

        public Dictionary<string, string> GetModuleCommandPairs()
        {
            return implantCommandValidation_Base.GetModuleCommandPairs();
        }

        public List<CommandItem> DisplayHelp(Dictionary<string, string> input)
        {
            return implantCommandValidation_Base.DisplayHelp(input);
        }

        public List<string> GetOptionalCommandList()
        {
            return implantCommandValidation_Base.GetOptionalCommandList();
        }

        public List<string> GetRequiredCommandList()
        {
            return implantCommandValidation_Base.GetRequiredCommandList();
        }

        public List<string> GetContextChangingCommands()
        {
            return implantCommandValidation_Base.GetContextChangingCommands();
        }


        //this should be overridden with the command list for the implant
        public List<CommandItem> CommandList { get; } = new List<CommandItem>()
        {
	        //new CommandItem()
	        //{
		       // Name = "CommandName",
		       // Description = "details about the command",
		       // Usage = "CommandName",
		       // NeedsAdmin = false,
		       // Opsec = OpsecStatus.NotSet,
		       // MitreTechnique = "",
		       // RequiresPreProc = false, //if true, there should be an override of the team servers ExtImplant_TaskPreProcess_Base class 
		       // RequiresPostProc = false, //if true, there should be an override of the team servers ExtImplant_TaskPostProcess_Base class 
		       // Keys = null,
            //},

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
