using ApiModels.Plugin_BaseClasses;
using Microsoft.AspNetCore.Http;
using System;
using System.Collections.Generic;
using System.ComponentModel.Composition;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using TeamServer.Plugin_BaseClasses;
using TeamServer.Utilities;

namespace Rivet_ServerPlugin
{
    [Export(typeof(ExtImplantHandleComms_Base))]
    [ExportMetadata("Name", "Rivet")]
    [ExportMetadata("Description", "This is the rivet implant handle comms")]
    public class Rivet_HandleComms_Base : ExtImplantHandleComms_Base
    {
        public override async Task HandlePostRequest(ExtImplant_Base implant, ExtImplantService_Base extImpService_base, HttpContext httpcontext)
        {
            Console.WriteLine($"{DateTime.UtcNow} handling POST request");
            byte[] Data;
            using var ms = new System.IO.MemoryStream();
            await httpcontext.Request.Body.CopyToAsync(ms);
            Data = ms.ToArray();

            IEnumerable<ExtImplantTaskResult_Base> processedTasks = await ProcessTaskResults(Data.Deserialize<IEnumerable<ExtImplantTaskResult_Base>>(), implant);
            await SendTaskResults(implant, processedTasks);
        }
    }
}
