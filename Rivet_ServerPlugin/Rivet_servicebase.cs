using HardHatCore.TeamServer.Plugin_BaseClasses;
using HardHatCore.ApiModels.Plugin_Interfaces;
using HardHatCore.TeamServer.Utilities;
using HardHatCore.ApiModels.Shared;
using HardHatCore.TeamServer.Models;
using HardHatCore.TeamServer.Services;
using System.Diagnostics;
using System.ComponentModel.Composition; //make sure to add this and not the System.Composition package
using HardHatCore.TeamServer.Plugin_Interfaces.Ext_Implants;
using Microsoft.AspNetCore.Http;

namespace Rivet_ServerPlugin
{
    [Export(typeof(IExtImplantService))]
    [ExportMetadata("Name", "Rivet")]
    public class Rivet_serviceBase : IExtImplantService
    {
        private readonly ExtImplantService_Base extImplantService_Base = new();


        public void AddExtImplant(ExtImplant_Base Implant)
        {
            extImplantService_Base.AddExtImplant(Implant);
        }

        public IEnumerable<ExtImplant_Base> GetExtImplants()
        {
            return extImplantService_Base.GetExtImplants();
        }

        public ExtImplant_Base GetExtImplant(string id)
        {
            return extImplantService_Base.GetExtImplant(id);
        }

        public void RemoveExtImplant(ExtImplant_Base Implant)
        {
            extImplantService_Base.RemoveExtImplant(Implant);
        }
        public bool AddExtImplantToDatabase(ExtImplant_Base implant)
        {
            return extImplantService_Base.AddExtImplantToDatabase(implant);
        }

        public Httpmanager GetImplantsManager(IExtImplantMetadata extImplantMetadata)
        {
            return extImplantService_Base.GetImplantsManager(extImplantMetadata);
        }

        public ExtImplant_Base InitImplantObj(IExtImplantMetadata implantMeta, ref HttpContext httpcontentxt, string pluginName)
        {
            return extImplantService_Base.InitImplantObj(implantMeta, ref httpcontentxt, pluginName);
        }

        public ExtImplant_Base InitImplantObj(IExtImplantMetadata implantMeta, string pluginName)
        {
            return extImplantService_Base.InitImplantObj(implantMeta, pluginName);
        }

        public void LogImplantFirstCheckin(ExtImplant_Base implant)
        {
            extImplantService_Base.LogImplantFirstCheckin(implant);
        }

        public void UpdateImplantDBInfo(ExtImplant_Base implant)
        {
            extImplantService_Base.UpdateImplantDBInfo(implant);
        }

        public void GenerateUniqueEncryptionKeys(string implantId)
        {
            extImplantService_Base.GenerateUniqueEncryptionKeys(implantId);
        }
        public byte[] HandleP2PDataDecryption(IExtImplant implant, byte[] encryptedData)
        {
            return extImplantService_Base.HandleP2PDataDecryption(implant, encryptedData);
        }

        //This has to be overridden because it is implant specific, in this case I find the src folder on Rivet and compile it using cargo 
        public bool CreateExtImplant(IExtImplantCreateRequest request, out string result_message)
        {
            Console.WriteLine("Rivet implant creation called");
            //this should still be Location/HardHatC2/Teamserver/ if the plugin is in the right place
            string basePath = HardHatCore.TeamServer.Utilities.Helpers.GetBaseFolderLocation();
            //this then should be Location/HardHatC2/Rivet/
            string rivetFolder = basePath + Helpers.PathingTraverseUpString + "Assets" + Helpers.PlatPathSeperator + "RivetAsset" + Helpers.PlatPathSeperator;
            //next up i need the src folder so i can make some string replacements with dynamic info 
            string srcFolder = rivetFolder + "src" + Helpers.PlatPathSeperator;

            string mainFile = File.ReadAllText($"{srcFolder}main.rs");
            // we should make a copy of main.rs that we can restore after compile 
            string mainFileCopy = mainFile;
            File.WriteAllText($"{srcFolder}main.rs.bak", mainFileCopy);

            Httpmanager manager = new();
            foreach (manager m in managerService._managers)
            {
                if (m.Type == ManagerType.http && m.Name == request.managerName)
                {
                    manager = (Httpmanager)m;
                }
            }
            //inside of this main file are some strings that need to be replaced 
            //implant names need to be xor'd before being updated
            mainFile = mainFile.Replace("{{REPLACE_IMPLANT_TYPE}}", Encryption.EncryptImplantName(request.implantType));
            mainFile = mainFile.Replace("{{REPLACE_SLEEP_TIME}}", request.Sleep.ToString());
            //  mainFile = mainFile.Replace("{{REPLACE_URLS}}", manager.c2Profile.Urls);
            // mainFile = mainFile.Replace("{{REPLACE_COOKIES}}", manager.c2Profile.Cookies);
            //mainFile = mainFile.Replace("{{REPLACE_REQUEST_HEADERS}}", manager.c2Profile.RequestHeaders);
            mainFile = mainFile.Replace("{{REPLACE_USERAGENT}}", manager.c2Profile.UserAgent);
            mainFile = mainFile.Replace("{{REPLACE_CONNECTION_IP}}", manager.ConnectionAddress);
            mainFile = mainFile.Replace("{{REPLACE_CONNECTION_PORT}}", manager.ConnectionPort.ToString());
            //now for rust we dont compile from reading the source as a string like with roslyn instead we need to save this back to disk
            File.WriteAllText($"{srcFolder}main.rs", mainFile);

            //get the metadata file
            string metadataFile = File.ReadAllText($"{srcFolder}models{Helpers.PlatPathSeperator}rustyMetadata.rs");
            //make a copy of the metadata file
            string metadataFileCopy = metadataFile;
            File.WriteAllText($"{srcFolder}models{Helpers.PlatPathSeperator}rustyMetadata.rs.bak", metadataFileCopy);
            //replace the {{REPLACE_MANAGER_NAME}} with the manager name
            metadataFile = metadataFile.Replace("{{REPLACE_MANAGER_NAME}}", request.managerName);
            //now save the metadata file back to disk
            File.WriteAllText($"{srcFolder}models{Helpers.PlatPathSeperator}rustyMetadata.rs", metadataFile);

            string OsBuildOption = "";
            if(request.implantOsType.Equals("Windows",StringComparison.CurrentCultureIgnoreCase))
            {
                OsBuildOption = "x86_64-pc-windows-msvc";
            }

            //use a create cargo to compile the implant by calling cargo build
            //this is the same as running cargo build from the command line
            Console.WriteLine("Starting cargo build");
            string error = "";
            Process cargo = new();
            cargo.StartInfo.FileName = "cargo";
            cargo.StartInfo.Arguments = $"build -r --target={OsBuildOption}";
            cargo.StartInfo.WorkingDirectory = rivetFolder;
            cargo.StartInfo.UseShellExecute = false;
            cargo.StartInfo.RedirectStandardOutput = true;
            cargo.StartInfo.RedirectStandardError = true;
            //redirect output to console
            cargo.OutputDataReceived += (sender, args) => Console.WriteLine(args.Data);
            cargo.ErrorDataReceived += (sender, args) => { error += args.Data; Console.WriteLine(args.Data); };

            cargo.Start();
            cargo.WaitForExit();


            cargo.Close();
            Console.WriteLine("Finished Cargo Build");
            //restore the overriden files
            File.WriteAllText($"{srcFolder}main.rs", mainFileCopy);
            File.WriteAllText($"{srcFolder}models{Helpers.PlatPathSeperator}rustyMetadata.rs", metadataFileCopy);

            if(error.Length ==0)
            {
                result_message = $"Successfully compiled rivet implant, check {rivetFolder}target{Helpers.PlatPathSeperator}{OsBuildOption}release{Helpers.PlatPathSeperator}";
                File.Copy($"{rivetFolder}target{Helpers.PlatPathSeperator}{OsBuildOption}{Helpers.PlatPathSeperator}release{Helpers.PlatPathSeperator}rivet.exe", $"{basePath}{Helpers.PathingTraverseUpString}Rivet_{request.managerName}.exe",true);
                return true;
            }
            else
            {
                result_message = "Failed to compile rivet implant";
                return false;
            }

        }

        //should be overriden to implement the same encryption as the implant 
        public byte[] EncryptImplantTaskData(byte[] bytesToEnc, string encryptionKey)
        {
            //rivet does not currently support encryption so we just return the bytesToEnc
            return bytesToEnc;
        }

        //should be overriden to implement the same encryption as the implant 
		public byte[] DecryptImplantTaskData(byte[] bytesToDec, string encryptionKey)
        {
            //rivet does not currently support encryption so we just return the bytesToDec
            return bytesToDec;
        }

    }
}