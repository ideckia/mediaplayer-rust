import api.media.IMediaPlayer;
import js.node.ChildProcess;
import js.node.child_process.ChildProcess as ChildProcessObject;

class RustMediaPlayer implements IMediaPlayer {
	var processes:Map<Int, ChildProcessObject>;

	static var executablePath:String;

	public function new() {
		processes = [];
		var filename = (Sys.systemName() == "Windows") ? 'rsmp.exe' : 'rsmp';

		executablePath = haxe.io.Path.join([js.Node.__dirname, 'lib', filename]);

		trace('Checking rsmp is installed.');
		var status = Sys.command(executablePath);
		if (status != 0)
			throw new haxe.Exception('This IMediaPlayer implementation needs the "lib/$filename" file to run.');
	}

	public function play(path:String, loop:Bool = false, ?onEnd:Void->Void) {
		var options:ChildProcessSpawnOptions = {
			shell: true
		}

		var cmd = '$executablePath --path $path';
		if (loop)
			cmd += ' --loop';

		var process = ChildProcess.spawn(cmd, options);
		var pid = process.pid;

		processes.set(pid, process);

		process.on('close', () -> {
			if (onEnd != null)
				onEnd();
			processes.remove(pid);
		});
		process.on('error', (e) -> {
			trace('Error: $e');
			processes.remove(pid);
		});

		return pid;
	}

	public function pause(id:Int) {
		sendCommand(id, 'pause');
	}

	public function resume(id:Int) {
		sendCommand(id, 'resume');
	}

	public function stop(id:Int) {
		sendCommand(id, 'stop');
	}

	function sendCommand(id:Int, cmd:String) {
		var process = processes.get(id);
		if (process == null)
			return;

		process.stdin.write('$cmd\n');
	}

	function killProcess(pid:Int, signal:String = 'SIGKILL') {
		if (Sys.systemName() == "Windows") {
			ChildProcess.exec('taskkill /PID ${pid} /T /F', (error, _, _) -> {
				if (error != null) {
					trace('RustMediaPlayer', 'Error killing process: $error');
				}
			});
		} else {
			// see https://nodejs.org/api/child_process.html#child_process_options_detached
			// If pid is less than -1, then sig is sent to every process in the process group whose ID is -pid.
			js.Node.process.kill(-pid, 'SIGKILL');
		}
	}
}
