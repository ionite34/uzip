"""Training for compression dictionary."""

from pathlib import Path

from git import Repo, RemoteProgress
from glob import glob
from tqdm import tqdm
from pyzstd import train_dict, finalize_dict

# Repositories to train on
REPOS = [
    # Requests: Apache 2.0
    "https://github.com/psf/requests",
    # Black: MIT
    "https://github.com/psf/black",
    # Numpy: BSD 3-Clause
    "https://github.com/numpy/numpy",
    # Attrs: MIT
    "https://github.com/python-attrs/attrs",
    # Click: BSD 3-Clause
    "https://github.com/pallets/click",
    # PyO3: Apache 2.0
    "https://github.com/PyO3/pyo3",
]


class CloneProgress(RemoteProgress):
    def __init__(self):
        super().__init__()
        self.pbar = tqdm()

    def update(self, op_code, cur_count, max_count=None, message=''):
        self.pbar.total = max_count
        self.pbar.n = cur_count
        self.pbar.refresh()


class Trainer:
    def __init__(self):
        self.path = Path('.train')
        self.path.mkdir(exist_ok=True)
        # Train on first 128KB of file, as per zstd limits
        self.file_max = 256 * 1024

    def get_samples(self):
        """Get samples from the repositories."""
        for repo in REPOS:
            print(f"Cloning [{repo}]...")
            name = repo.split('/')[-1]
            path = self.path / name
            if not path.exists():
                Repo.clone_from(repo, path, progress=CloneProgress())
            else:
                repo = Repo(path)
                repo.remotes.origin.pull(progress=CloneProgress())

    def train(self):
        """Train the dictionary."""
        for repo in REPOS:
            name = repo.split('/')[-1]
            path = self.path / name
            print(f"Training [{name}]...")
            # files = tqdm(list(path.rglob('**/*.py')))
            files = tqdm(
                p.resolve() for p in Path(path).glob("**/*")
                if p.suffix in {".py", ".c", ".cpp", ".h", ".rs", ".js", ".toml", ".ini"}
            )
            for file in files:
                files.set_description(file.name)
                with open(file, 'rb') as f:
                    # data = f.read(self.file_max)
                    data = f.read()
                yield data


def main():
    dict_size = 1000 * 1024  # 1MB
    t = Trainer()
    t.get_samples()

    data = train_dict(t.train(), dict_size)

    data = finalize_dict(data, t.train(), dict_size, 21)
    with open('dict.bin', 'wb') as f:
        f.write(data.dict_content)

    print(f"Done | Dict Size: {len(data.dict_content)}")


if __name__ == '__main__':
    main()
