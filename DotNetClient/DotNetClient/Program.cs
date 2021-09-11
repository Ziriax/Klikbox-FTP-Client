using System;
using System.IO;
using System.Linq;
using System.Net;
using System.Threading;
using System.Threading.Tasks;
using FluentFTP;
using Microsoft.Extensions.Configuration;

namespace DotNetClient
{
	public class Settings
	{
		public string Ip { get; set; }
		public int Port { get; set; }
		public string Src { get; set; }
		public string Dst { get; set; }
		public string User { get; set; }
		public string Password { get; set; }
		public int ReadyDelayMS { get;set;}
	}

	class Program
	{
		static async Task Main(string[] args)
		{
			var settings = GetSettings();

			for (;;)
			{
				await MoveFiles(settings, Console.WriteLine);
				Thread.Sleep(1000);
			}
		}

		static async Task MoveFiles(Settings settings, Action<string> onMessage)
		{
			try
			{
				onMessage("Creating output directory...");
				Directory.CreateDirectory(settings.Dst);
				
				var credentials = new NetworkCredential(settings.User, settings.Password);
				using var ftp = new FtpClient(settings.Ip, credentials)
				{
					Port = settings.Port
				};

				onMessage("Connecting to FTP server...");

				await ftp.AutoConnectAsync();

				onMessage("Listing FTP files...");

				var listing1 = (await ftp.GetListingAsync(settings.Src)).ToDictionary(item => item.Name);
				
				onMessage($"Waiting {settings.ReadyDelayMS/1000D} seconds...");

				// Wait 5 seconds, and retrieve listing again. 
				// Files that are not modified are assumed ready for download.
				await Task.Delay(settings.ReadyDelayMS);
				
				onMessage("Finding FTP files ready for download...");

				var listing2 = (await ftp.GetListingAsync(settings.Src))
					.Where(item2 => listing1.TryGetValue(item2.Name, out var item1) && item1.Modified == item2.Modified)
					.ToArray();
				
				foreach (var item2 in listing2)
				{
					onMessage($"Downloading {item2.FullName}...");

					var status = await ftp.DownloadFileAsync(
						Path.Combine(settings.Dst, item2.Name),
						item2.FullName, 
						FtpLocalExists.Overwrite);

					onMessage("\n");
					
					if (status == FtpStatus.Success)
					{
						onMessage($"Deleting {item2.FullName}...");
						await ftp.DeleteFileAsync(item2.FullName);
					}
					else
					{
						onMessage($"Failed to download {item2.FullName}: {status}!");
					}
				}
			}
			catch (Exception ex)
			{
				Console.WriteLine(ex.ToString());
			}
		}

		static Settings GetSettings()
		{
			var builder = new ConfigurationBuilder()
				.SetBasePath(Directory.GetCurrentDirectory())
				.AddJsonFile("appsettings.json", optional: false, reloadOnChange: true);

			IConfiguration config = builder.Build();

			var settings = config.GetSection("FTP").Get<Settings>();
			return settings;
		}

		private sealed class DownloadProgress : IProgress<FtpProgress>
		{
			private readonly Action<double> _progress;

			public DownloadProgress(Action<double> progress)
			{
				_progress  = progress;
			}
			
			public void Report(FtpProgress value)
			{
				_progress(value.Progress);
			}
		}
	}
}