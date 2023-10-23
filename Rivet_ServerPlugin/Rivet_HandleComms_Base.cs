using HardHatCore.ApiModels.Plugin_BaseClasses;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;
using System.ComponentModel.Composition;
using HardHatCore.ApiModels.Plugin_Interfaces;
using HardHatCore.TeamServer.Models; //make sure to add this and not the System.Composition package
using HardHatCore.TeamServer.Plugin_BaseClasses;
using HardHatCore.TeamServer.Utilities;
using HardHatCore.TeamServer.Plugin_Interfaces.Ext_Implants;

namespace Rivet_ServerPlugin
{
    [Export(typeof(IExtimplantHandleComms))]
    [ExportMetadata("Name", "Rivet")]
    [ExportMetadata("Description", "This is the rivet implant handle comms")]
    public class Rivet_HandleComms_Base : IExtimplantHandleComms
    {
        private readonly ExtImplantHandleComms_Base extImplantHandleComms_Base = new();

        //overriden as an example since the default performs encryption and decryption and Rivet does not need that
        public async Task HandlePostRequest(ExtImplant_Base implant, ExtImplantService_Base extImpService_base, HttpContext httpcontext)
        {
            Console.WriteLine($"{DateTime.UtcNow} handling POST request from rivet");
            byte[] Data;
            using var ms = new System.IO.MemoryStream();
            await httpcontext.Request.Body.CopyToAsync(ms);
            Data = ms.ToArray();

            IEnumerable<ExtImplantTaskResult_Base> processedTasks = await ProcessTaskResults(Data.Deserialize<IEnumerable<ExtImplantTaskResult_Base>>(), implant);
            await SendTaskResults(implant, processedTasks);
        }

        public async Task<ExtImplant_Base> GetCheckingInImplant(IExtImplantMetadata extImplantmetadata, HttpContext httpContext, IExtImplantService extImplantService_Base, string pluginName)
        {
            return await extImplantHandleComms_Base.GetCheckingInImplant(extImplantmetadata, httpContext, extImplantService_Base, pluginName);
        }

        public async Task HandleImplantRequest(ExtImplant_Base extImplant, IExtImplantService extImplantService_Base, HttpContext httpContext)
        {
            await extImplantHandleComms_Base.HandleImplantRequest(extImplant, extImplantService_Base, httpContext);
        }

        public async Task<IActionResult> RespondToImplant(ExtImplant_Base extImplant, IExtImplantService extImplantService_Base)
        {
            return await extImplantHandleComms_Base.RespondToImplant(extImplant, extImplantService_Base);
        }

        public async Task<byte[]> ReturnImplantTasking(ExtImplant_Base extImplant, IExtImplantService extImplantService_Base)
        {
            return await extImplantHandleComms_Base.ReturnImplantTasking(extImplant, extImplantService_Base);
        }

        public bool SetHttpResponseHeaders(ref HttpContext httpContext, Httpmanager httpmanager)
        {
            return extImplantHandleComms_Base.SetHttpResponseHeaders(ref httpContext, httpmanager);
        }

        public void CreateNewP2PPath(string implantId)
        {
            extImplantHandleComms_Base.CreateNewP2PPath(implantId);
        }

        public async Task HandleGetRequest(ExtImplant_Base implant)
        {
            await extImplantHandleComms_Base.HandleGetRequest(implant);
        }

        public async Task HandlePostRequest(ExtImplant_Base implant, IExtImplantService extImpService_base, HttpContext copiedHttpContext)
        {
            await HandlePostRequest(implant, extImpService_base, copiedHttpContext);
        }

        public async Task<IEnumerable<ExtImplantTaskResult_Base>> ProcessTaskResults(IEnumerable<ExtImplantTaskResult_Base> taskResults, ExtImplant_Base implant)
        {
            return await extImplantHandleComms_Base.ProcessTaskResults(taskResults, implant);
        }

        public async Task SendTaskResults(ExtImplant_Base implant, IEnumerable<ExtImplantTaskResult_Base> results)
        {
            await extImplantHandleComms_Base.SendTaskResults(implant, results);
        }

        public async Task<byte[]> HandlePreProcAndPackageTasking(ExtImplant_Base implant)
        {
            return await extImplantHandleComms_Base.HandlePreProcAndPackageTasking(implant);
        }

        public void CreateNewEncKeyAndTaskUpdate(ExtImplant_Base implant, IExtImplantService extImplantService_Base)
        {
            extImplantHandleComms_Base.CreateNewEncKeyAndTaskUpdate(implant, extImplantService_Base);
        }
    }
}
