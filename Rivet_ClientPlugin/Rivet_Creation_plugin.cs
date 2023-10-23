using HardHatCore.HardHatC2Client.Components.ImplantCreation;
using HardHatCore.HardHatC2Client.Plugin_BaseClasses;
using HardHatCore.HardHatC2Client.Plugin_Interfaces;
using Microsoft.AspNetCore.Components;
using System.ComponentModel.Composition; //make sure to add this and not the System.Composition package


namespace Rivet_ClientPlugin
{
    [Export(typeof(IimplantCreation))]
    //metadata that is used to config which build options are displayed in the UI
    [ImplantCreationBaseData(
            Name = "Rivet",
            Description = "This is the rivet implant creation",
            SupportedCommTypes = new string[] { "HTTP" },
            SupportedOperatingSystems = new string[] { "Windows" },
            SupportedOutputTypes = new string[] { "exe" },
            SupportsConnectionAttempts = true,
            SupportsKillDates = false,
            SupportsPostEx = false
    )]
    public class Rivet_Creation_plugin : IimplantCreation
    {
        //just needs to be set with the name of the .Razor file that is used for the UI when creating the implant
        public Type GetComponentType()
        {
            return typeof(Rivet_creation_comp);
        }

        //does not have a module options UI element it needs to load so just returns an empty builder
        public RenderFragment GetModuleOptionsUI()
        {
            return builder => { };

            //if you want to add a module options UI element, uncomment the below and add your component in the typeof call
            //return builder =>
            //{
            //    builder.OpenComponent(0, typeof(ModuleOptionsComponent));
            //    builder.CloseComponent();
            //};
        }
    }
}
